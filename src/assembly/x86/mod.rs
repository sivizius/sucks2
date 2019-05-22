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
        length
        = match instruction.getType()
          {
            InstructionType::Label          ( identifier )
            =>  {
                  if identifier >= self.identifiers.len()
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
                  labels[ identifier ]  =   address;
                  Some  ( 0 )
                },
            InstructionType::AAA        =>  { instruction.setOpcode ( 0x37  );  Some  ( 1 ) },
            InstructionType::AAD        =>  unimplemented!(),
            InstructionType::AAM        =>  unimplemented!(),
            InstructionType::AAS        =>  { instruction.setOpcode ( 0x3f  );  Some  ( 1 ) },
            InstructionType::ADC        =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x10, )?,
            InstructionType::ADD        =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x00, )?,
            InstructionType::AND        =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x20, )?,
            InstructionType::CALL       =>  unimplemented!(),
            InstructionType::CBW        =>  { instruction.setOpcode ( 0x98  );  Some  ( 1 ) },
            InstructionType::CLC        =>  { instruction.setOpcode ( 0xf8  );  Some  ( 1 ) },
            InstructionType::CLD        =>  { instruction.setOpcode ( 0xfc  );  Some  ( 1 ) },
            InstructionType::CLI        =>  { instruction.setOpcode ( 0xfa  );  Some  ( 1 ) },
            InstructionType::CMC        =>  { instruction.setOpcode ( 0xf5  );  Some  ( 1 ) },
            InstructionType::CMP        =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x38, )?,
            InstructionType::CMPSB      =>  { instruction.setOpcode ( 0xa6  );  Some  ( 1 ) },
            InstructionType::CMPSW      =>  { instruction.setOpcode ( 0xa7  );  Some  ( 1 ) },
            InstructionType::CWD        =>  { instruction.setOpcode ( 0x99  );  Some  ( 1 ) },
            InstructionType::DAA        =>  { instruction.setOpcode ( 0x27  );  Some  ( 1 ) },
            InstructionType::DAS        =>  { instruction.setOpcode ( 0x2f  );  Some  ( 1 ) },
            InstructionType::DEC        =>  unimplemented!(),
            InstructionType::DIV        =>  unimplemented!(),
            InstructionType::ESC        =>  unimplemented!(),
            InstructionType::HLT        =>  { instruction.setOpcode ( 0xf4  );  Some  ( 1 ) },
            InstructionType::IDIV       =>  unimplemented!(),
            InstructionType::IMUL       =>  unimplemented!(),
            InstructionType::IN         =>  unimplemented!(),
            InstructionType::INC        =>  unimplemented!(),
            InstructionType::INT        =>  unimplemented!(),
            InstructionType::INT3       =>  { instruction.setOpcode ( 0xcc  );  Some  ( 1 ) },
            InstructionType::INTO       =>  { instruction.setOpcode ( 0xce  );  Some  ( 1 ) },
            InstructionType::IRET       =>  { instruction.setOpcode ( 0xcf  );  Some  ( 1 ) },
            InstructionType::JB         =>  unimplemented!(),
            InstructionType::JBE        =>  unimplemented!(),
            InstructionType::JCXZ       =>  unimplemented!(),
            InstructionType::JE         =>  unimplemented!(),
            InstructionType::JL         =>  unimplemented!(),
            InstructionType::JLE        =>  unimplemented!(),
            InstructionType::JMP        =>  unimplemented!(),
            InstructionType::JNB        =>  unimplemented!(),
            InstructionType::JNBE       =>  unimplemented!(),
            InstructionType::JNE        =>  unimplemented!(),
            InstructionType::JNL        =>  unimplemented!(),
            InstructionType::JNLE       =>  unimplemented!(),
            InstructionType::JNO        =>  unimplemented!(),
            InstructionType::JNP        =>  unimplemented!(),
            InstructionType::JNS        =>  unimplemented!(),
            InstructionType::JO         =>  unimplemented!(),
            InstructionType::JP         =>  unimplemented!(),
            InstructionType::JS         =>  unimplemented!(),
            InstructionType::LAHF       =>  { instruction.setOpcode ( 0x9f  );  Some  ( 1 ) },
            InstructionType::LDS        =>  unimplemented!(),
            InstructionType::LEA        =>  unimplemented!(),
            InstructionType::LES        =>  unimplemented!(),
            InstructionType::LODSB      =>  { instruction.setOpcode ( 0xac  );  Some  ( 1 ) },
            InstructionType::LODSW      =>  { instruction.setOpcode ( 0xad  );  Some  ( 1 ) },
            InstructionType::LOOP       =>  unimplemented!(),
            InstructionType::LOOPZ      =>  unimplemented!(),
            InstructionType::LOOPNZ     =>  unimplemented!(),
            InstructionType::MOV        =>  unimplemented!(),
            InstructionType::MOVSB      =>  { instruction.setOpcode ( 0xa4  );  Some  ( 1 ) },
            InstructionType::MOVSW      =>  { instruction.setOpcode ( 0xa5  );  Some  ( 1 ) },
            InstructionType::OR         =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x08, )?,
            InstructionType::MUL        =>  unimplemented!(),
            InstructionType::NEG        =>  unimplemented!(),
            InstructionType::NOT        =>  unimplemented!(),
            InstructionType::OUT        =>  unimplemented!(),
            InstructionType::POP        =>  unimplemented!(),
            InstructionType::POPF       =>  { instruction.setOpcode ( 0x9d  );  Some  ( 1 ) },
            InstructionType::PUSH       =>  unimplemented!(),
            InstructionType::PUSHF      =>  { instruction.setOpcode ( 0x9c  );  Some  ( 1 ) },
            InstructionType::RCL        =>  unimplemented!(),
            InstructionType::RCR        =>  unimplemented!(),
            InstructionType::RETF       =>  unimplemented!(),
            InstructionType::RETN       =>  unimplemented!(),
            InstructionType::ROL        =>  unimplemented!(),
            InstructionType::ROR        =>  unimplemented!(),
            InstructionType::SAHF       =>  { instruction.setOpcode ( 0x9e  );  Some  ( 1 ) },
            InstructionType::SAL        =>  unimplemented!(),
            InstructionType::SALC       =>  { instruction.setOpcode ( 0xd6  );  Some  ( 1 ) },
            InstructionType::SAR        =>  unimplemented!(),
            InstructionType::SBB        =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x18, )?,
            InstructionType::SCASB      =>  { instruction.setOpcode ( 0xae  );  Some  ( 1 ) },
            InstructionType::SCASW      =>  { instruction.setOpcode ( 0xaf  );  Some  ( 1 ) },
            InstructionType::SHL        =>  unimplemented!(),
            InstructionType::SHR        =>  unimplemented!(),
            InstructionType::STC        =>  { instruction.setOpcode ( 0xf9  );  Some  ( 1 ) },
            InstructionType::STD        =>  { instruction.setOpcode ( 0xfd  );  Some  ( 1 ) },
            InstructionType::STI        =>  { instruction.setOpcode ( 0xfb  );  Some  ( 1 ) },
            InstructionType::STOSB      =>  { instruction.setOpcode ( 0xaa  );  Some  ( 1 ) },
            InstructionType::STOSW      =>  { instruction.setOpcode ( 0xab  );  Some  ( 1 ) },
            InstructionType::SUB        =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x28, )?,
            InstructionType::TEST       =>  unimplemented!(),
            InstructionType::WAIT       =>  { instruction.setOpcode ( 0xdb  );  Some  ( 1 ) },
            InstructionType::XCHG       =>  unimplemented!(),
            InstructionType::XLAT       =>  { instruction.setOpcode ( 0xd7  );  Some  ( 1 ) },
            InstructionType::XOR        =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x30, )?,
            _                           =>  panic!  ( "Unexpected Instruction. This should not happen here!"  ),
          };
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
