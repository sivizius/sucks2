#[macro_use]
pub mod expressions;
mod instructions;
#[macro_use]
pub mod memory;
mod operands;
pub mod registers;

pub use self::
{
  instructions::
  {
    AddressSizeOverride,
    BranchTaken,
    BranchNotTaken,
    Instruction,
    InstructionAddress,
    InstructionType,
    Lock,
    OperandSizeOverride,
    Repeat,
    RepeatEqual,
    RepeatZero,
    RepeatNotEqual,
    RepeatNotZero,
    SegmentOverrideCS,
    SegmentOverrideSS,
    SegmentOverrideDS,
    SegmentOverrideES,
    SegmentOverrideFS,
    SegmentOverrideGS,
    ThreeByteXOP,
    TwoByteVEX,
    ThreeByteVEX,
    TwoByteOpcode,
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
        self.features,
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

      //  try to resolve expressions
      let mut size                      =   0;
      for operand                       in  instruction.getOperandRefs()
      {
        if let OperandType::Expression ( expression ) = operand
        {
          let ( newSize,  newOperand  ) =   expression.solve()?;
          *operand                      =   newOperand;
          if let  Some  ( newSize  ) = newSize
          {
            size                        |=  newSize;
          }
          else
          {
            length                      =   None;
          }
          //println!  ( "{:?}",  *operand );
        }
      }
      instruction.orOperandSize ( size  );

      //  if not possible, skip further processing of instruction
      if length != None
      {
        match instruction.getType()
        {
          InstructionType::Label          ( identifier )
          =>  {
                if identifier < self.identifiers.len()
                {
                  labels[ identifier ]  =   address;
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
                length                  =   Some  ( 0 );
              },
          InstructionType::AAA          =>  { instruction.setOpcode ( 0x37  );  length  = Some  ( 1 ) },
          InstructionType::AAS          =>  { instruction.setOpcode ( 0x3f  );  length  = Some  ( 1 ) },
          InstructionType::ADC          =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  0x10, )?,
          InstructionType::ADD          =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  0x00, )?,
          InstructionType::AND          =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  0x20, )?,
          InstructionType::CBW          =>  { instruction.setOpcode ( 0x98  );  length  = Some  ( 1 ) },
          InstructionType::CLC          =>  { instruction.setOpcode ( 0xf8  );  length  = Some  ( 1 ) },
          InstructionType::CLD          =>  { instruction.setOpcode ( 0xfc  );  length  = Some  ( 1 ) },
          InstructionType::CLI          =>  { instruction.setOpcode ( 0xfa  );  length  = Some  ( 1 ) },
          InstructionType::CMC          =>  { instruction.setOpcode ( 0xf5  );  length  = Some  ( 1 ) },
          InstructionType::CMP          =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  0x38, )?,
          InstructionType::CMPSB        =>  { instruction.setOpcode ( 0xa6  );  length  = Some  ( 1 ) },
          InstructionType::CWD          =>  { instruction.setOpcode ( 0x99  );  length  = Some  ( 1 ) },
          InstructionType::DAA          =>  { instruction.setOpcode ( 0x27  );  length  = Some  ( 1 ) },
          InstructionType::DAS          =>  { instruction.setOpcode ( 0x2f  );  length  = Some  ( 1 ) },
          InstructionType::HLT          =>  { instruction.setOpcode ( 0xf4  );  length  = Some  ( 1 ) },
          InstructionType::INT3         =>  { instruction.setOpcode ( 0xcc  );  length  = Some  ( 1 ) },
          InstructionType::INTO         =>  { instruction.setOpcode ( 0xce  );  length  = Some  ( 1 ) },
          InstructionType::IRET         =>  { instruction.setOpcode ( 0xcf  );  length  = Some  ( 1 ) },
          InstructionType::LAHF         =>  { instruction.setOpcode ( 0x9f  );  length  = Some  ( 1 ) },
          InstructionType::LODSB        =>  { instruction.setOpcode ( 0xac  );  length  = Some  ( 1 ) },
          InstructionType::MOVSB        =>  { instruction.setOpcode ( 0xa4  );  length  = Some  ( 1 ) },
          InstructionType::OR           =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  0x08, )?,
          InstructionType::POPF         =>  { instruction.setOpcode ( 0x9d  );  length  = Some  ( 1 ) },
          InstructionType::PUSHF        =>  { instruction.setOpcode ( 0x9c  );  length  = Some  ( 1 ) },
          InstructionType::SAHF         =>  { instruction.setOpcode ( 0x9e  );  length  = Some  ( 1 ) },
          InstructionType::SALC         =>  { instruction.setOpcode ( 0xd6  );  length  = Some  ( 1 ) },
          InstructionType::SBB          =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  0x18, )?,
          InstructionType::SCASB        =>  { instruction.setOpcode ( 0xae  );  length  = Some  ( 1 ) },
          InstructionType::STC          =>  { instruction.setOpcode ( 0xf9  );  length  = Some  ( 1 ) },
          InstructionType::STD          =>  { instruction.setOpcode ( 0xfd  );  length  = Some  ( 1 ) },
          InstructionType::STI          =>  { instruction.setOpcode ( 0xfb  );  length  = Some  ( 1 ) },
          InstructionType::STOSB        =>  { instruction.setOpcode ( 0xaa  );  length  = Some  ( 1 ) },
          InstructionType::SUB          =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  0x28, )?,
          InstructionType::WAIT         =>  { instruction.setOpcode ( 0xdb  );  length  = Some  ( 1 ) },
          InstructionType::XLAT         =>  { instruction.setOpcode ( 0xd7  );  length  = Some  ( 1 ) },
          InstructionType::XOR          =>  length  = X86::compileSimpleMathInstruction ( &mut instruction, architecture, operandSize,  addressSize,  0x30, )?,
          _
          =>  {
                instruction.print();
                return  Err
                        (
                          "Unexpected Instruction. This should not happen here!".to_string()
                        );
              },
        }
      }

      //  address calculations
      instruction.setAddress  ( address );
      address.add             ( length  )?;

      //  debug instruction
      instruction.print       ();
    }

    //  and finally encode all teh things
    for instruction                     in  self.instructions
    {
      if let Some ( opcode ) = instruction.getOpcode()
      {
        //  Group 1
        if instruction.hazLock()                  { output.push ( Lock                              ); }
        if instruction.hazRepeat()                { output.push ( instruction.getRepeat()           ); }

        //  Group 2
        if instruction.hazSegmentOverride()       { output.push ( instruction.getSegmentOverride()  ); }
        if instruction.hazBranchHint()            { output.push ( instruction.getBranchHint()       ); }

        //  Group 3
        if instruction.hazOperandSizeOverride()   { output.push ( OperandSizeOverride               ); }

        //  Group 4
        if instruction.hazAddressSizeOverride()   { output.push ( AddressSizeOverride               ); }

        if instruction.hazThreeByteXOP()          { output.push ( ThreeByteXOP                      ); }
        if instruction.hazTwoByteVEX()            { output.push ( TwoByteVEX                        ); }
        if instruction.hazThreeByteVEX()          { output.push ( ThreeByteVEX                      ); }
        if instruction.hazREX()                   { output.push ( instruction.getREX()              ); }

        //  Opcode
        if instruction.hazTwoByteOpcode()         { output.push ( TwoByteOpcode                     ); }
        output.push ( opcode );

        //  Mod Reg R/M
        if let Some ( value ) = instruction.getModRegRM()
        {
          output.push ( value );
        }

        //  Scale Index Base
        if let Some ( value ) = instruction.getSIBByte()
        {
          output.push ( value );
        }

        //  Displacement Value
        let ( length, displacement  )             =   instruction.getDisplacement();
        for ctr                                   in  0 .. length
        {
          output.push ( ( ( displacement >> ( 8 * ctr ) ) & 0xff ) as u8 );
        }

        //  Immediate Value
        let ( length, immediate )                 =   instruction.getImmediate();
        for ctr                                   in  0 .. length
        {
          output.push ( ( ( immediate >> ( 8 * ctr ) ) & 0xff ) as u8 );
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
