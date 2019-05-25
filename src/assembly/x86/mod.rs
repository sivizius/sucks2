#[macro_use]
pub mod expressions;
mod instructions;
#[macro_use]
pub mod memory;
mod operands;
pub mod registers;
pub mod symbols;

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
  symbols::
  {
    Symbol,
    SymbolList,
  },
};

pub use super::
{
  AssemblyFeatures,
  InstructionSet,
};

use std::
{
  string::
  {
    String,
  },
};

pub struct X86
{
  instructions:                         Vec<Instruction>,
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
    line:                               0,
    features:                           AssemblyFeatures::Default,
  }
}

impl X86
{
  pub fn label
  (
    mut self,
    name:                               &'static str,
  ) -> Self
  {
    self.instructions.push
    (
      Instruction
      (
        self.line,
        self.features,
        0,
        InstructionType::Label ( name ),
        vec!(),
      )
    );
    self.line                           += 1;
    self
  }

  #[allow(unused_mut)]
  pub fn compile
  (
    mut self,
    mut architecture:                   InstructionSet,
    mut operandSize:                    usize,
    mut addressSize:                    usize,
    maxRounds:                          usize,
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

    let mut symbols                     =   SymbolList  ( );
    let mut rounds                      =   None;
    for round                           in  0 .. maxRounds
    {
      println!  ( "round: {}",  round );
      let mut done                      =   true;
      let mut address                   =   InstructionAddress
                                            {
                                              base: 0,
                                              offs: 0,
                                            };

      //  for every instruction: try to compile
      for mut instruction               in  &mut self.instructions
      {
        let mut length                  =   Some ( 0 );

        //  try to resolve expressions and labels
        let mut size                    =   0;
        for operand                     in  instruction.getOperandRefs()
        {
          if    let OperandType::Expression ( expression  ) = operand
          {
            let
            (
              newSize,
              newOperand,
            )                           =   expression.solve()?;
            *operand                    =   newOperand;
            if let  Some  ( newSize  ) = newSize
            {
              size                      |=  newSize;
            }
            else
            {
              length                    =   None;
            }
            //println!  ( "{:?}",  *operand );
          }
          else
          {
            if  let OperandType::Symbol     ( identifier  ) = operand
            {
              let reference             =   symbols.expect  ( identifier  );
              *operand                  =   OperandType::Reference  ( reference );
            }
            if  let OperandType::Reference  ( reference   ) = operand
            {
              if let  Some ( value )
                  =   symbols.obtain
                      (
                        *reference,
                        round,
                      )
              {
                match value
                {
                  OperandType::Address  ( destination )
                  =>  if let Some ( displacement  ) = address.diff  ( destination )
                      {
                        *operand        =   OperandType::Displacement ( displacement  );
                      },
                  _
                  =>  *operand          =   value,
                }
              }
              else
              {
                done                    =   false;
              }
            }
          }
        }
        instruction.orOperandSize ( size  );

        //  if not possible, skip further processing of instruction
        if length != None
        {
          //  minimum length, instruction might be longer
          length
          = match instruction.getType()
            {
              InstructionType::Label          ( identifier  )
              =>  if let  Ok ( reference )
                      =   symbols.define
                          (
                            identifier,
                            Some  ( OperandType::Address  ( address ) ),
                            round,
                          )
                  {
                    instruction.setType
                    (
                      InstructionType::Reference  ( reference ),
                    );
                    Ok  ( Some  ( 0 ) )
                  }
                  else
                  {
                    Err ( "Label already defined".to_string ( ) )
                  },
              InstructionType::Reference      ( reference   )
              =>  if let  Some ( error )
                      =   symbols.modify
                          (
                            reference,
                            Some  ( OperandType::Address  ( address ) ),
                            round,
                          )
                  {
                    Err ( error.to_string ( ) )
                  }
                  else
                  {
                    Ok  ( Some  ( 0 ) )
                  },
              InstructionType::AAA      =>  instruction.compileZeroOperandInstruction (                                           0x37, ),
              InstructionType::AAD      =>  unimplemented!(),
              InstructionType::AAM      =>  unimplemented!(),
              InstructionType::AAS      =>  instruction.compileZeroOperandInstruction (                                           0x3f, ),
              InstructionType::ADC      =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x10, ),
              InstructionType::ADD      =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x00, ),
              InstructionType::AND      =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x20, ),
              InstructionType::CALL     =>  unimplemented!(),
              InstructionType::CBW      =>  instruction.compileZeroOperandInstruction (                                           0x98, ),
              InstructionType::CLC      =>  instruction.compileZeroOperandInstruction (                                           0xf8, ),
              InstructionType::CLD      =>  instruction.compileZeroOperandInstruction (                                           0xfc, ),
              InstructionType::CLI      =>  instruction.compileZeroOperandInstruction (                                           0xfa, ),
              InstructionType::CMC      =>  instruction.compileZeroOperandInstruction (                                           0xf5, ),
              InstructionType::CMP      =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x38, ),
              InstructionType::CMPSB    =>  instruction.compileZeroOperandInstruction (                                           0xa6, ),
              InstructionType::CMPSW    =>  instruction.compileZeroOperandInstruction (                                           0xa7, ),
              InstructionType::CWD      =>  instruction.compileZeroOperandInstruction (                                           0x99, ),
              InstructionType::DAA      =>  instruction.compileZeroOperandInstruction (                                           0x27, ),
              InstructionType::DAS      =>  instruction.compileZeroOperandInstruction (                                           0x2f, ),
              InstructionType::DEC      =>  unimplemented!(),
              InstructionType::DIV      =>  unimplemented!(),
              InstructionType::ESC      =>  unimplemented!(),
              InstructionType::HLT      =>  instruction.compileZeroOperandInstruction (                                           0xf4, ),
              InstructionType::IDIV     =>  unimplemented!(),
              InstructionType::IMUL     =>  unimplemented!(),
              InstructionType::IN       =>  unimplemented!(),
              InstructionType::INC      =>  unimplemented!(),
              InstructionType::INT      =>  unimplemented!(),
              InstructionType::INT3     =>  instruction.compileZeroOperandInstruction (                                           0xcc, ),
              InstructionType::INTO     =>  instruction.compileZeroOperandInstruction (                                           0xce, ),
              InstructionType::IRET     =>  instruction.compileZeroOperandInstruction (                                           0xcf, ),
              InstructionType::JB       =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x72, ),
              InstructionType::JBE      =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x76, ),
              InstructionType::JCXZ     =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0xe3, ),
              InstructionType::JE       =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x74, ),
              InstructionType::JL       =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x7c, ),
              InstructionType::JLE      =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x7e, ),
              InstructionType::JMP      =>  unimplemented!(),
              InstructionType::JNB      =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x73, ),
              InstructionType::JNBE     =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x77, ),
              InstructionType::JNE      =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x75, ),
              InstructionType::JNL      =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x7d, ),
              InstructionType::JNLE     =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x7f, ),
              InstructionType::JNO      =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x71, ),
              InstructionType::JNP      =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x7b, ),
              InstructionType::JNS      =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x79, ),
              InstructionType::JO       =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x70, ),
              InstructionType::JP       =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x7a, ),
              InstructionType::JS       =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0x78, ),
              InstructionType::LAHF     =>  instruction.compileZeroOperandInstruction (                                           0x9f, ),
              InstructionType::LDS      =>  unimplemented!(),
              InstructionType::LEA      =>  unimplemented!(),
              InstructionType::LES      =>  unimplemented!(),
              InstructionType::LODSB    =>  instruction.compileZeroOperandInstruction (                                           0xac, ),
              InstructionType::LODSW    =>  instruction.compileZeroOperandInstruction (                                           0xad, ),
              InstructionType::LOOP     =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0xe2, ),
              InstructionType::LOOPZ    =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0xe1, ),
              InstructionType::LOOPNZ   =>  instruction.compileJumpInstruction        ( architecture, operandSize,                0xe0, ),
              InstructionType::MOV      =>  unimplemented!(),
              InstructionType::MOVSB    =>  instruction.compileZeroOperandInstruction (                                           0xa4, ),
              InstructionType::MOVSW    =>  instruction.compileZeroOperandInstruction (                                           0xa5, ),
              InstructionType::OR       =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x08, ),
              InstructionType::MUL      =>  unimplemented!(),
              InstructionType::NEG      =>  unimplemented!(),
              InstructionType::NOT      =>  unimplemented!(),
              InstructionType::OUT      =>  unimplemented!(),
              InstructionType::POP      =>  unimplemented!(),
              InstructionType::POPF     =>  instruction.compileZeroOperandInstruction (                                           0x9d, ),
              InstructionType::PUSH     =>  unimplemented!(),
              InstructionType::PUSHF    =>  instruction.compileZeroOperandInstruction (                                           0x9c, ),
              InstructionType::RCL      =>  unimplemented!(),
              InstructionType::RCR      =>  unimplemented!(),
              InstructionType::RETF     =>  unimplemented!(),
              InstructionType::RETN     =>  unimplemented!(),
              InstructionType::ROL      =>  unimplemented!(),
              InstructionType::ROR      =>  unimplemented!(),
              InstructionType::SAHF     =>  instruction.compileZeroOperandInstruction (                                           0x9e, ),
              InstructionType::SAL      =>  unimplemented!(),
              InstructionType::SALC     =>  instruction.compileZeroOperandInstruction (                                           0xd6, ),
              InstructionType::SAR      =>  unimplemented!(),
              InstructionType::SBB      =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x18, ),
              InstructionType::SCASB    =>  instruction.compileZeroOperandInstruction (                                           0xae, ),
              InstructionType::SCASW    =>  instruction.compileZeroOperandInstruction (                                           0xaf, ),
              InstructionType::SHL      =>  unimplemented!(),
              InstructionType::SHR      =>  unimplemented!(),
              InstructionType::STC      =>  instruction.compileZeroOperandInstruction (                                           0xf9, ),
              InstructionType::STD      =>  instruction.compileZeroOperandInstruction (                                           0xfd, ),
              InstructionType::STI      =>  instruction.compileZeroOperandInstruction (                                           0xfb, ),
              InstructionType::STOSB    =>  instruction.compileZeroOperandInstruction (                                           0xaa, ),
              InstructionType::STOSW    =>  instruction.compileZeroOperandInstruction (                                           0xab, ),
              InstructionType::SUB      =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x28, ),
              InstructionType::TEST     =>  unimplemented!(),
              InstructionType::WAIT     =>  instruction.compileZeroOperandInstruction (                                           0xdb, ),
              InstructionType::XCHG     =>  unimplemented!(),
              InstructionType::XLAT     =>  instruction.compileZeroOperandInstruction (                                           0xd7, ),
              InstructionType::XOR      =>  instruction.compileSimpleMathInstruction  ( architecture, operandSize,  addressSize,  0x30, ),
              _                         =>  panic!  ( "Unexpected Instruction. This should not happen here!"  ),
            }?;
        }

        //  address calculations
        instruction.setAddress  ( address );
        address.add             ( length  );
      }
      if  done
      &&  address.done ( )
      {
        rounds                          =   Some  ( round );
        break;
      }
    }

    if let Some ( rounds  ) = rounds
    {
      //  and finally encode all teh things
      for instruction                   in  self.instructions
      {
        instruction.print       ();
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
          let ( length, displacement  ) =   instruction.getDisplacement();
          for ctr                       in  0 .. length
          {
            output.push ( ( ( displacement >> ( 8 * ctr ) ) & 0xff ) as u8 );
          }

          //  Immediate Value
          let ( length, immediate )     =   instruction.getImmediate();
          for ctr                       in  0 .. length
          {
            output.push ( ( ( immediate >> ( 8 * ctr ) ) & 0xff ) as u8 );
          }
        }
      }
      Ok  ( output.into_boxed_slice() )
    }
    else
    {
      Err
      (
        format!
        (
          "Cannot be Compiled in {} Rounds. Either the Code Just Cannot be Compiled or You Have to Adjust the Number of Rounds",
          maxRounds,
        )
      )
    }
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
