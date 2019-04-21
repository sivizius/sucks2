#[macro_use]
mod instructions;
mod operands;
#[macro_use]
mod registers;

pub use self::
{
  instructions::*,
  operands::*,
  registers::*,
};

pub use super::
{
  InstructionSet,
};

use rand;

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
  }
}

impl X86
{
  GeneralPurposeRegister! ( al,  1,  0 );
  GeneralPurposeRegister! ( cl,  1,  1 );
  GeneralPurposeRegister! ( dl,  1,  2 );
  GeneralPurposeRegister! ( bl,  1,  3 );
  GeneralPurposeRegister! ( ah,  1,  4 );
  GeneralPurposeRegister! ( ch,  1,  5 );
  GeneralPurposeRegister! ( dh,  1,  6 );
  GeneralPurposeRegister! ( bh,  1,  7 );
  GeneralPurposeRegister! ( ax,  2,  0 );
  GeneralPurposeRegister! ( cx,  2,  1 );
  GeneralPurposeRegister! ( dx,  2,  2 );
  GeneralPurposeRegister! ( bx,  2,  3 );
  GeneralPurposeRegister! ( sp,  1,  4 );
  GeneralPurposeRegister! ( bp,  1,  5 );
  GeneralPurposeRegister! ( si,  1,  6 );
  GeneralPurposeRegister! ( di,  1,  7 );
  GeneralPurposeRegister! ( eax, 4,  0 );
  GeneralPurposeRegister! ( ecx, 4,  1 );
  GeneralPurposeRegister! ( edx, 4,  2 );
  GeneralPurposeRegister! ( ebx, 4,  3 );
  GeneralPurposeRegister! ( esp, 4,  4 );
  GeneralPurposeRegister! ( ebp, 4,  5 );
  GeneralPurposeRegister! ( esi, 4,  6 );
  GeneralPurposeRegister! ( edi, 4,  7 );

  SegmentRegister!        ( cs,      0 );
  SegmentRegister!        ( ss,      1 );
  SegmentRegister!        ( ds,      2 );
  SegmentRegister!        ( es,      3 );
  SegmentRegister!        ( fs,      4 );
  SegmentRegister!        ( gs,      5 );

  ControlRegister!        ( cr0,     0 );
  ControlRegister!        ( cr1,     1 );
  ControlRegister!        ( cr2,     2 );
  ControlRegister!        ( cr3,     3 );
  ControlRegister!        ( cr4,     4 );
  ControlRegister!        ( cr5,     5 );
  ControlRegister!        ( cr6,     6 );
  ControlRegister!        ( cr7,     7 );

  DebugRegister!          ( dr0,     0 );
  DebugRegister!          ( dr1,     1 );
  DebugRegister!          ( dr2,     2 );
  DebugRegister!          ( dr3,     3 );
  DebugRegister!          ( dr4,     4 );
  DebugRegister!          ( dr5,     5 );
  DebugRegister!          ( dr6,     6 );
  DebugRegister!          ( dr7,     7 );

  TestRegister!           ( tr6,     6 );
  TestRegister!           ( tr7,     7 );

  MulitMediaRegister!     ( mm0,     0 );
  MulitMediaRegister!     ( mm1,     1 );
  MulitMediaRegister!     ( mm2,     2 );
  MulitMediaRegister!     ( mm3,     3 );
  MulitMediaRegister!     ( mm4,     4 );
  MulitMediaRegister!     ( mm5,     5 );
  MulitMediaRegister!     ( mm6,     6 );
  MulitMediaRegister!     ( mm7,     7 );

  simpleMathInstruction!  ( add,  0x00 );
  simpleMathInstruction!  ( or,   0x08 );
  simpleMathInstruction!  ( adc,  0x10 );
  simpleMathInstruction!  ( sbb,  0x18 );
  simpleMathInstruction!  ( and,  0x20 );
  simpleMathInstruction!  ( sub,  0x28 );
  simpleMathInstruction!  ( xor,  0x30 );
  simpleMathInstruction!  ( cmp,  0x38 );

  pub fn label
  (
    mut self,
    name:                               &str,
  ) -> Result<Self, &'static str>
  {
    self.instructions.push
    (
      Instruction
      {
        line:                           self.line,
        mnemonic:                       "label",
        size:                           0,
        length:                         Some ( 0 ),
        opcode:                         InstructionType::Label ( self.identifiers.len() ),
        operands:                       vec!(),
      }
    );
    self.line                           += 1;
    self.identifiers.insert
    (
      String::from( name ),
      self.identifiers.len()
    );
    Ok ( self )
  }

  fn simpleMathInstruction
  (
    mut self,
    opcode:                             InstructionType,
    mnemonic:                           &'static str,
    dst:                                impl Operand,
    src:                                impl Operand,
  ) -> Result<Self, &'static str>
  {
    let ( dstThis, dstSize )            =   dst.this();
    let ( srcThis, srcSize )            =   src.this();
    let size: usize                     =   dstSize | srcSize;
    if          size == 0
    {
      Err ( "Operand Size not Specified" )
    }
    else  if  ( size != 1 )
          &&  ( size != 2 )
    {
      Err ( "Invalid Operand Sizes" )
    }
    else
    {
      self.instructions.push
      (
        Instruction
        {
          line:                         self.line,
          mnemonic:                     mnemonic,
          size:                         size,
          length:                       None,
          opcode:                       opcode,
          operands:                     vec!( dstThis, srcThis ),
        }
      );
      self.line                         +=  1;
      Ok ( self )
    }
  }

  #[allow(unused_mut)]
  pub fn compile
  (
    self,
    mut architecture:                   InstructionSet,
    mut operandSize:                    usize,
    mut addressSize:                    usize,
  ) -> Result<usize, String>
  {
    if  ( architecture < InstructionSet::i386 )
    &&  (
          ( operandSize != 16 ) || ( addressSize != 16 )
        )
    {
      return Err ( format!( "Instruction Set ›{}‹ is 16 Bit Only", InstructionSet( architecture ) ) );
    }

    let mut labels: Vec<Option<usize>>  =   vec!();
    labels.resize
    (
      self.identifiers.len(),
      None
    );

    let mut address:          usize     =   0;
    for instruction in self.instructions
    {
      match instruction.opcode
      {
        InstructionType::Label          ( identifier )
        =>  {
              if identifier < self.identifiers.len()
              {
                labels[ identifier ]    =   Some ( address );
              }
              else
              {
                return Err ( format! ( "Invalid Label Number ›{}‹ on Line {}", identifier, instruction.line ) );
              }
            },
        InstructionType::SimpleMath     ( opcode )
        =>  {
              if instruction.operands.len() == 2
              {
                match ( &instruction.operands [ 0 ], &instruction.operands [ 1 ] )
                {
                  ( OperandType::GeneralPurposeRegister ( dstRegister ),  OperandType::Constant               ( immediate ) )
                  =>  {
                        if  ( *dstRegister == 0 )
                        &&  ( true || rand::random() )
                        {
                          match instruction.size
                          {
                            1
                            =>  {
                                  if  *immediate >= -0x80
                                  &&  *immediate <=  0xff
                                  {
                                  }
                                  else
                                  {
                                    return Err
                                            (
                                              format!
                                              (
                                                "Value Out of Bonds [-0x80,0xff] {}",
                                                *immediate,
                                              )
                                            );
                                  }
                                },
                            2
                            =>  {
                                  if  *immediate >= -0x8000
                                  &&  *immediate <=  0xffff
                                  {
                                  }
                                  else
                                  {
                                    return Err
                                            (
                                              format!
                                              (
                                                "Value Out of Bonds [-0x8000,0xffff] {}",
                                                *immediate,
                                              )
                                            );
                                  }
                                },
                            4 //if operandSize > 16
                            =>  {
                                  if  *immediate >= -0x80000000
                                  &&  *immediate <=  0xffffffff
                                  {
                                  }
                                  else
                                  {
                                    return Err
                                            (
                                              format!
                                              (
                                                "Value Out of Bonds [-0x80000000,0xffffffff] {}",
                                                *immediate,
                                              )
                                            );
                                  }
                                },
                            _
                            =>  {
                                  return  Err
                                          (
                                            format!
                                            (
                                              "Invalid Operand Size {}",
                                              instruction.size,
                                            )
                                          );
                                },
                            
                          }
                        }
                      },
                  ( OperandType::GeneralPurposeRegister ( dstRegister ),  OperandType::GeneralPurposeRegister ( srcRegister ) )
                  =>  {
                        
                      },
                  ( _, _ )
                  =>  {
                        return  Err
                                (
                                  format!
                                  (
                                    "Invalid Combination of Arguments ›{}‹, ›{}‹ for Instruction ›{}‹",
                                    instruction.operands [ 0 ].to_string(),
                                    instruction.operands [ 1 ].to_string(),
                                    instruction.mnemonic,
                                  )
                                );
                      },
                }
              }
              else
              {
                return Err ( format! ( "Instruction ›{}‹ Must Take Exactly 2 Arguments", instruction.mnemonic ) );
              }
            },
        _
        =>  {
            },
      }
    }

    Ok(2)
  }
}
