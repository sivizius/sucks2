#[macro_use]
mod instructions;
mod operands;
#[macro_use]
mod registers;

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
