use super::
{
  Instruction,
  InstructionAddress,
  InstructionPrefix,
  InstructionType,
  super::
  {
    AssemblyFeatures,
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
    features:                           AssemblyFeatures,
    opcode:                             u8,
  ) -> Result<Option<usize>, String>
  {
    if instruction.operands.len() == 2
    {
      match ( &instruction.operands [ 0 ], &instruction.operands [ 1 ] )
      {
        ( OperandType::GeneralPurposeRegister ( mut dstRegister ),  OperandType::Constant               ( mut immediate ) )
        =>  {
              if  ( dstRegister == 0 )
              && !( features.hazFeature ( AssemblyFeatures::RandomOpcodeSize ) && rand::random() )
              {
                match instruction.size
                {
                  1
                  =>  {
                        if  immediate >= -0x80
                        &&  immediate <=  0xff
                        {
                          instruction.setOpcode   (         InstructionType::OneByte  ( opcode | dstRegister | 4  )   );
                          instruction.setOperands ( vec!  ( OperandType::Byte         ( immediate                 ) ) );
                          Ok ( Some ( 2 ) )
                        }
                        else
                        {
                          Err
                          (
                            format!
                            (
                              "Value Out of Bonds [-0x80,0xff] {} on Line {}",
                              immediate,
                              instruction.getLineNumber(),
                            )
                          )
                        }
                      },
                  2
                  =>  {
                        if  immediate >= -0x8000
                        &&  immediate <=  0xffff
                        {
                          if *operandSize == 32
                          {
                            instruction.addPrefix ( InstructionPrefix::OperandSizeOverride );
                          }
                          instruction.setOpcode   (         InstructionType::OneByte  ( opcode | dstRegister | 5  )   );
                          instruction.setOperands ( vec!  ( OperandType::Word         ( immediate                 ) ) );
                          Ok ( Some ( 3 ) )
                        }
                        else
                        {
                          Err
                          (
                            format!
                            (
                              "Value Out of Bonds [-0x8000,0xffff] {} on Line {}",
                              immediate,
                              instruction.getLineNumber(),
                            )
                          )
                        }
                      },
                  4 if *architecture >= InstructionSet::i386
                  =>  {
                        if  immediate >= -0x80000000
                        &&  immediate <=  0xffffffff
                        {
                          if *operandSize == 16
                          {
                             instruction.addPrefix ( InstructionPrefix::OperandSizeOverride );
                          }
                          instruction.setOpcode   (         InstructionType::OneByte  ( opcode | dstRegister | 5  )   );
                          instruction.setOperands ( vec!  ( OperandType::DWord        ( immediate                 ) ) );
                          Ok ( Some ( 5 ) )
                        }
                        else
                        {
                          Err
                          (
                            format!
                            (
                              "Value Out of Bonds [-0x80000000,0xffffffff] {} on Line {}",
                              immediate,
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
                Ok ( None )
              }
            },
        ( OperandType::GeneralPurposeRegister ( dstRegister ),  OperandType::GeneralPurposeRegister ( srcRegister ) )
        =>  {
              Ok ( None )
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