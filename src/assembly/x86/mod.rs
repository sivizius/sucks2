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
      for part                          in  instruction.getParts()
      {
        match part
        {
          InstructionPart::Lock                   =>  output.push ( 0xf0 ),
          InstructionPart::Repeat                 =>  output.push ( 0xf3 ),
          InstructionPart::RepeatNot              =>  output.push ( 0xf2 ),
          InstructionPart::SegmentOverridePrefix  ( segment   )
          =>  {
                match segment
                {
                  0                               =>  output.push ( 0x26 ),  //  CS
                  1                               =>  output.push ( 0x2e ),  //  SS
                  2                               =>  output.push ( 0x36 ),  //  DS
                  3                               =>  output.push ( 0x3e ),  //  ES
                  4                               =>  output.push ( 0x64 ),  //  FS
                  5                               =>  output.push ( 0x65 ),  //  GS
                  _                               =>  return Err ( format! ( "Invalid Segment Value {}", segment ) ),
                }
              },
          InstructionPart::BranchTaken            =>  output.push ( 0x3e ),
          InstructionPart::BranchNotTaken         =>  output.push ( 0x2e ),
          InstructionPart::OperandSizeOverride    =>  output.push ( 0x66 ),
          InstructionPart::AddressSizeOverride    =>  output.push ( 0x67 ),
          InstructionPart::ThreeByteXOP           =>  output.push ( 0x8f ),
          InstructionPart::TwoByteVEX             =>  output.push ( 0xc5 ),
          InstructionPart::ThreeByteVEX           =>  output.push ( 0xc4 ),
          InstructionPart::REXPrefix              ( value     )
          =>  output.push ( 0x40 | ( value & 0x0f ) ),
          InstructionPart::OneByteInstruction     ( opcode    )
          =>  {
                output.push ( opcode  );
              }
          InstructionPart::TwoByteInstruction     ( opcode    )
          =>  {
                output.push ( 0x0f    );
                output.push ( opcode  );
              },
          InstructionPart::ModRegRM               ( value     )
          =>  output.push ( value ),
          InstructionPart::SIBByte                ( value     )
          =>  output.push ( value ),
          InstructionPart::ImmediateByte          ( immediate )
          =>  {
                output.push ( ( ( immediate >>  0 ) & 0xff ) as u8 );
              },
          InstructionPart::ImmediateWord          ( immediate )
          =>  {
                output.push ( ( ( immediate >>  0 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >>  8 ) & 0xff ) as u8 );
              },
          InstructionPart::ImmediateDWord         ( immediate )
          =>  {
                output.push ( ( ( immediate >>  0 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >>  8 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 16 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 24 ) & 0xff ) as u8 );
              },
          InstructionPart::ImmediateQWord         ( immediate )
          =>  {
                output.push ( ( ( immediate >>  0 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >>  8 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 16 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 24 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 32 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 40 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 48 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 56 ) & 0xff ) as u8 );
              },
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
