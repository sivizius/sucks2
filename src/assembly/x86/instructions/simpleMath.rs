use super::
{
  Instruction,
  InstructionPrefix,
  InstructionType,
  super::
  {
    InstructionSet,
    X86,
    operands::
    {
      Operand,
      OperandType,
    },
  },
};

use rand;

macro_rules! simpleMathInstruction
{
  (
    $theName:ident,
    $theOpcode:expr
  )
  =>  {
        pub fn $theName
        (
          self,
          dst:                          impl Operand,
          src:                          impl Operand,
        ) -> Self
        {
          self.simpleMathInstruction ( InstructionType::SimpleMath ( $theOpcode ), stringify! ( $theName ), dst, src )
        }
      }
}

impl X86
{
  simpleMathInstruction!  ( add,  0x00 );
  simpleMathInstruction!  ( or,   0x08 );
  simpleMathInstruction!  ( adc,  0x10 );
  simpleMathInstruction!  ( sbb,  0x18 );
  simpleMathInstruction!  ( and,  0x20 );
  simpleMathInstruction!  ( sub,  0x28 );
  simpleMathInstruction!  ( xor,  0x30 );
  simpleMathInstruction!  ( cmp,  0x38 );

  fn simpleMathInstruction
  (
    mut self,
    opcode:                             InstructionType,
    mnemonic:                           &'static str,
    dst:                                impl Operand,
    src:                                impl Operand,
  ) -> Self
  {
    let ( dstThis, dstSize )            =   dst.this();
    let ( srcThis, srcSize )            =   src.this();
    let size: usize                     =   dstSize | srcSize;
    self.instructions.push
    (
      Instruction
      (
        self.line,
        mnemonic,
        size,
        None,
        opcode,
        vec! ( dstThis, srcThis ),
      )
    );
    self.line                           +=  1;
    self
  }

  pub(in super::super) fn compileSimpleMathInstruction
  (
    instruction:                        &mut Instruction,
    architecture:                       &mut InstructionSet,
    operandSize:                        &mut usize,
    addressSize:                        &mut usize,
    opcode:                             u8,
  ) -> Result<(), String>
  {
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
                          instruction.setOpcode ( InstructionType::OneByte ( *dstRegister | 4  ) );
                          Ok (())
                        }
                        else
                        {
                          Err
                          (
                            format!
                            (
                              "Value Out of Bonds [-0x80,0xff] {} on Line {}",
                              *immediate,
                              instruction.getLineNumber(),
                            )
                          )
                        }
                      },
                  2
                  =>  {
                        if  *immediate >= -0x8000
                        &&  *immediate <=  0xffff
                        {
                          if *operandSize == 32
                          {
                            instruction.addPrefix ( InstructionPrefix::OperandSizeOverride );
                          }
                          Ok (())
                        }
                        else
                        {
                          Err
                          (
                            format!
                            (
                              "Value Out of Bonds [-0x8000,0xffff] {} on Line {}",
                              *immediate,
                              instruction.getLineNumber(),
                            )
                          )
                        }
                      },
                  4 if *architecture >= InstructionSet::i386
                  =>  {
                        if  *immediate >= -0x80000000
                        &&  *immediate <=  0xffffffff
                        {
                          if *operandSize == 16
                          {
                            instruction.prefixes.push ( InstructionPrefix::OperandSizeOverride );
                          }
                          Ok (())
                        }
                        else
                        {
                          Err
                          (
                            format!
                            (
                              "Value Out of Bonds [-0x80000000,0xffffffff] {} on Line {}",
                              *immediate,
                              instruction.getLineNumber(),
                            )
                          )
                        }
                      },
                  0
                  =>  {
                        Err
                        (
                          format!
                          (
                            "Operand Size not Specified on Line {}",
                            instruction.getLineNumber(),
                          )
                        )
                      },
                  _
                  =>  {
                        Err
                        (
                          format!
                          (
                            "Invalid Operand Size {} on Line {}",
                            instruction.size,
                            instruction.getLineNumber(),
                          )
                        )
                      },
                  
                }
              }
              else
              {
                Ok (())
              }
            },
        ( OperandType::GeneralPurposeRegister ( dstRegister ),  OperandType::GeneralPurposeRegister ( srcRegister ) )
        =>  {
              Ok (())
            },
        ( _, _ )
        =>  {
              Err
              (
                format!
                (
                  "Invalid Combination of Arguments ›{}‹, ›{}‹ for Instruction ›{}‹ on Line {}",
                  instruction.operands [ 0 ].to_string(),
                  instruction.operands [ 1 ].to_string(),
                  instruction.getMnemonic(),
                  instruction.getLineNumber(),
                )
              )
            },
      }
    }
    else
    {
      Err
      (
        format!
        (
          "Instruction ›{}‹ Must Take Exactly 2 Arguments {} on Line {}",
          instruction.getMnemonic(),
          instruction.operands.len(),
          instruction.getLineNumber(),
        )
      )
    }
  }
}