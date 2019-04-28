#[macro_use]
mod instructions;
mod operands;
#[macro_use]
mod registers;

pub use self::
{
  instructions::
  {
    Instruction,
    InstructionAddress,
    InstructionPart,
    InstructionType,
  },
  operands::
  {
    OperandType,
  },
};

pub use super::
{
  AssemblyFeatures,
  InstructionSet,
};

use std::
{
  collections::
  {
    HashMap,
  },
  string::
  {
    String,
  },
};

pub struct X86
{
  instructions:                         Vec<Instruction>,
  identifiers:                          HashMap<String, usize>,
  line:                                 usize,
  features:                             AssemblyFeatures,
}

pub fn X86
(
) -> X86
{
  X86
  {
    instructions:                       vec!(),
    identifiers:                        HashMap::new(),
    line:                               0,
    features:                           AssemblyFeatures::Default,
  }
}

impl X86
{
  pub fn label
  (
    mut self,
    name:                               &str,
  ) -> Self
  {
    self.instructions.push
    (
      Instruction
      (
        self.line,
        0,
        InstructionType::Label ( self.identifiers.len() ),
        vec!(),
      )
    );
    self.line                           += 1;
    self.identifiers.insert
    (
      String::from( name ),
      self.identifiers.len()
    );
    self
  }

  #[allow(unused_mut)]
  pub fn compile
  (
    mut self,
    mut architecture:                   InstructionSet,
    mut operandSize:                    usize,
    mut addressSize:                    usize,
  ) -> Result<Box<[u8]>, String>
  {
    let mut output:             Vec<u8> =   vec!();

    if  ( architecture < InstructionSet::i386 )
    &&  (
          ( operandSize != 16 ) || ( addressSize != 16 )
        )
    {
      return Err ( format!( "Instruction Set ›{}‹ is 16 Bit Only", InstructionSet( architecture ) ) );
    }

    let mut labels:                     Vec<InstructionAddress> 
                                        =   vec!();
    labels.resize
    (
      self.identifiers.len(),
      InstructionAddress::None
    );

    let mut address
    = InstructionAddress::Some
      {
        base:                           0,
        offs:                           0,
      };

    //  for every instruction: try to compile
    for mut instruction                 in  &mut self.instructions
    {
      let mut length                    =   Some ( 0 );
      match instruction.getType()
      {
        InstructionType::Label          ( identifier )
        =>  {
              if identifier < self.identifiers.len()
              {
                labels[ identifier ]    =   address;
              }
              else
              {
                instruction.print();
                return  Err
                        (
                          format!
                          (
                            "Invalid Label Number ›{}‹",
                            identifier,
                          )
                        );
              }
            },
        InstructionType::ADD            =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  self.features,  0x00, )?,
        InstructionType::OR             =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  self.features,  0x08, )?,
        InstructionType::ADC            =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  self.features,  0x10, )?,
        InstructionType::SBB            =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  self.features,  0x18, )?,
        InstructionType::AND            =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  self.features,  0x20, )?,
        InstructionType::SUB            =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  self.features,  0x28, )?,
        InstructionType::XOR            =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  self.features,  0x30, )?,
        InstructionType::CMP            =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  self.features,  0x38, )?,
        _
        =>  {
              instruction.print();
              return  Err
                      (
                        "Unexpected Instruction. This should not happen here!".to_string()
                      );
            },
      }

      instruction.setAddress  ( address );
      address.add             ( length  )?;
      instruction.setLength   ( length  );
      instruction.print       ();
    }

    //  and finally encode all teh parts
    for instruction                     in  self.instructions
    {
      let     parts                     =   instruction.getParts();
      if parts.len() > 0
      {
        let mut hazLock                 =   false   as bool;
        let mut hazRepeat               =   0       as u8;
        let mut hazSegmentOverride      =   0       as u8;
        let mut hazBranchHint           =   0       as u8;
        let mut hazOperandSizeOverride  =   false   as bool;
        let mut hazAddressSizeOverride  =   false   as bool;
        let mut hazThreeByteXOP         =   false   as bool;
        let mut hazTwoByteVEX           =   false   as bool;
        let mut hazThreeByteVEX         =   false   as bool;
        let mut hazREX                  =   0       as u8;
        let mut hazTwoByteOpcode        =   false   as bool;
        let mut theOpcode               =   0       as u8;
        let mut hazModRegRM             =   None    as Option<u8>;
        let mut hazSIBByte              =   None    as Option<u8>;
        let mut lstImmediate            =   vec!()  as Vec<u8>;
        for part                        in  parts
        {
          match part
          {
            InstructionPart::Lock                 =>  hazLock                   =   true,
            InstructionPart::Repeat               =>  hazRepeat                 =   0xf3,
            InstructionPart::RepeatNot            =>  hazRepeat                 =   0xf2,
            InstructionPart::SegmentOverride      ( segment   )
            =>  {
                  match segment
                  {
                    0                             =>  hazSegmentOverride        =   0x26, //  CS
                    1                             =>  hazSegmentOverride        =   0x2e, //  SS
                    2                             =>  hazSegmentOverride        =   0x36, //  DS
                    3                             =>  hazSegmentOverride        =   0x3e, //  ES
                    4                             =>  hazSegmentOverride        =   0x64, //  FS
                    5                             =>  hazSegmentOverride        =   0x65, //  GS
                    _                             =>  return Err ( format! ( "Invalid Segment Value {}", segment ) ),
                  }
                },
            InstructionPart::BranchTaken          =>  hazBranchHint             =   0x3e,
            InstructionPart::BranchNotTaken       =>  hazBranchHint             =   0x2e,
            InstructionPart::OperandSizeOverride  =>  hazOperandSizeOverride    =   true,
            InstructionPart::AddressSizeOverride  =>  hazAddressSizeOverride    =   true,
            InstructionPart::ThreeByteXOP         =>  hazThreeByteXOP           =   true,
            InstructionPart::TwoByteVEX           =>  hazTwoByteVEX             =   true,
            InstructionPart::ThreeByteVEX         =>  hazThreeByteVEX           =   true,
            InstructionPart::REX                  ( value     )
            =>  {
                  hazREX                =   0x40 | ( value & 0x0f );
                }
            InstructionPart::OneByteInstruction   ( opcode    )
            =>  {
                  theOpcode             =   opcode;
                }
            InstructionPart::TwoByteInstruction   ( opcode    )
            =>  {
                  hazTwoByteOpcode      =   true;
                  theOpcode             =   opcode;
                },
            InstructionPart::ModRegRM             ( value     )
            =>  {
                  hazModRegRM           =   Some ( value );
                },
            InstructionPart::SIBByte              ( value     )
            =>  {
                  hazSIBByte            =   Some ( value );
                },
            InstructionPart::ImmediateByte        ( immediate )
            =>  {
                  lstImmediate.push ( ( ( immediate >>  0 ) & 0xff ) as u8 );
                },
            InstructionPart::ImmediateWord        ( immediate )
            =>  {
                  lstImmediate.push ( ( ( immediate >>  0 ) & 0xff ) as u8 );
                  lstImmediate.push ( ( ( immediate >>  8 ) & 0xff ) as u8 );
                },
            InstructionPart::ImmediateDWord       ( immediate )
            =>  {
                  lstImmediate.push ( ( ( immediate >>  0 ) & 0xff ) as u8 );
                  lstImmediate.push ( ( ( immediate >>  8 ) & 0xff ) as u8 );
                  lstImmediate.push ( ( ( immediate >> 16 ) & 0xff ) as u8 );
                  lstImmediate.push ( ( ( immediate >> 24 ) & 0xff ) as u8 );
                },
            InstructionPart::ImmediateQWord       ( immediate )
            =>  {
                  lstImmediate.push ( ( ( immediate >>  0 ) & 0xff ) as u8 );
                  lstImmediate.push ( ( ( immediate >>  8 ) & 0xff ) as u8 );
                  lstImmediate.push ( ( ( immediate >> 16 ) & 0xff ) as u8 );
                  lstImmediate.push ( ( ( immediate >> 24 ) & 0xff ) as u8 );
                  lstImmediate.push ( ( ( immediate >> 32 ) & 0xff ) as u8 );
                  lstImmediate.push ( ( ( immediate >> 40 ) & 0xff ) as u8 );
                  lstImmediate.push ( ( ( immediate >> 48 ) & 0xff ) as u8 );
                  lstImmediate.push ( ( ( immediate >> 56 ) & 0xff ) as u8 );
                },
          }
        }
        if hazLock                        { output.push ( 0xf0                ); }
        if hazRepeat                !=  0 { output.push ( hazRepeat           ); }
        if hazSegmentOverride       !=  0 { output.push ( hazSegmentOverride  ); }
        if hazBranchHint            !=  0 { output.push ( hazBranchHint       ); }
        if hazOperandSizeOverride         { output.push ( 0x66                ); }
        if hazAddressSizeOverride         { output.push ( 0x67                ); }
        if hazThreeByteXOP                { output.push ( 0x8f                ); }
        if hazTwoByteVEX                  { output.push ( 0xc5                ); }
        if hazThreeByteVEX                { output.push ( 0xc4                ); }
        if hazREX                   !=  0 { output.push ( hazREX              ); }
        if hazTwoByteOpcode               { output.push ( 0x0f                ); }
        output.push ( theOpcode );
        if let Some ( value ) = hazModRegRM
        {
          output.push ( value );
        }
        if let Some ( value ) = hazSIBByte
        {
          output.push ( value );
        }
        for byte                          in  lstImmediate
        {
          output.push ( byte );
        }
      }
    }

    Ok ( output.into_boxed_slice() )
  }

  pub fn list
  (
    &self,
  )
  {
    for     instruction                 in  &self.instructions
    {
      instruction.print ();
    }
  }
}
