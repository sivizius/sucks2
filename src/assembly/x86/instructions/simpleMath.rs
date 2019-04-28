use super::
{
  Instruction,
  InstructionAddress,
  InstructionPart,
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

macro_rules!  declareSimpleMathInstruction
{
  (
    $theName:ident,
    $theInstruction:expr
  )
  =>  {
        pub fn $theName
        (
          self,
          dst:                          impl Operand,
          src:                          impl Operand,
        ) -> Self
        {
          self.simpleMathInstruction ( $theInstruction, dst, src )
        }
      }
}

macro_rules!  compileSimpleMathInstruction
{
  (
    $theOpcode:expr
  )
  =>  {
        length
        = X86::compileSimpleMathInstruction
          (
            &mut instruction,
            &mut architecture,
            &mut operandSize,
            &mut addressSize,
            self.features,
            $theOpcode,
          )?;
      }
}

impl X86
{
  declareSimpleMathInstruction! ( add,  InstructionType::ADD  );
  declareSimpleMathInstruction! ( or,   InstructionType::OR   );
  declareSimpleMathInstruction! ( adc,  InstructionType::ADC  );
  declareSimpleMathInstruction! ( sbb,  InstructionType::SBB  );
  declareSimpleMathInstruction! ( and,  InstructionType::AND  );
  declareSimpleMathInstruction! ( sub,  InstructionType::SUB  );
  declareSimpleMathInstruction! ( xor,  InstructionType::XOR  );
  declareSimpleMathInstruction! ( cmp,  InstructionType::CMP  );

  fn simpleMathInstruction
  (
    mut self,
    instruction:                        InstructionType,
    dst:                                impl Operand,
    src:                                impl Operand,
  ) -> Self
  {
    let ( dstThis, dstSize )            =   dst.this();
    let ( srcThis, srcSize )            =   src.this();
    let size                            =   ( dstSize | srcSize ) as usize;
    self.instructions.push
    (
      Instruction
      (
        self.line,
        size,
        instruction,
        vec! ( dstThis, srcThis ),
      )
    );
    self.line                           +=  1;
    self
  }

  pub(in super::super) fn compileSimpleMathInstruction
  (
    instruction:                        &mut Instruction,
    architecture:                       InstructionSet,
    operandSize:                        usize,
    addressSize:                        usize,
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
                          instruction.addPart ( InstructionPart::OneByteInstruction ( opcode | dstRegister | 4  ) );
                          instruction.addPart ( InstructionPart::ImmediateByte      ( immediate                 ) );
                          Ok ( Some ( 2 ) )
                        }
                        else
                        {
                          instruction.fail
                          (
                            format!
                            (
                              "Value Out of Bonds [-0x80,0xff]: {}",
                              immediate,
                            )
                          )
                        }
                      },
                  2
                  =>  {
                        if  immediate >= -0x8000
                        &&  immediate <=  0xffff
                        {
                          instruction.addPart ( InstructionPart::OneByteInstruction ( opcode | dstRegister | 5  ) );
                          instruction.addPart ( InstructionPart::ImmediateWord      ( immediate                 ) );
                          if operandSize == 32
                          {
                            instruction.addPart ( InstructionPart::OperandSizeOverride );
                            Ok ( Some ( 4 ) )
                          }
                          else
                          {
                            Ok ( Some ( 3 ) )
                          }
                        }
                        else
                        {
                          instruction.fail
                          (
                            format!
                            (
                              "Value Out of Bonds [-0x8000,0xffff] {}",
                              immediate,
                            )
                          )
                        }
                      },
                  4 if architecture >= InstructionSet::i386
                  =>  {
                        if  immediate >= -0x80000000
                        &&  immediate <=  0xffffffff
                        {
                          instruction.addPart ( InstructionPart::OneByteInstruction ( opcode | dstRegister | 5  ) );
                          instruction.addPart ( InstructionPart::ImmediateDWord     ( immediate                 ) );
                          if operandSize == 16
                          {
                            instruction.addPart ( InstructionPart::OperandSizeOverride );
                            Ok ( Some ( 6 ) )
                          }
                          else
                          {
                            Ok ( Some ( 5 ) )
                          }
                        }
                        else
                        {
                          instruction.fail
                          (
                            format!
                            (
                              "Value Out of Bonds [-0x80000000,0xffffffff] {}",
                              immediate,
                            )
                          )
                        }
                      },
                  0
                  =>  {
                        instruction.fail
                        (
                          format!
                          (
                            "Operand Size not Specified on Line",
                          )
                        )
                      },
                  _
                  =>  {
                        instruction.fail
                        (
                          format!
                          (
                            "Invalid Operand Size {}",
                            instruction.size,
                          )
                        )
                      },
                  
                }
              }
              else
              {
                if instruction.size == 1
                {
                  if  immediate >= -0x80
                  &&  immediate <=  0xff
                  {
                    let mut coin        =   0;
                    if  architecture < InstructionSet::amd64
                    &&  features.hazFeature ( AssemblyFeatures::RandomOpcode )
                    &&  rand::random()
                    {
                      coin              =   2;
                    }
                    //  0x80 and 0x82 are aliases, but 0x82 is invalid for 64 bit.
                    //  because 0x80 is the default encoding, some disassemblers fail with 0x82.
                    instruction.addPart ( InstructionPart::OneByteInstruction ( opcode    | coin | 0  ) );
                    instruction.addPart ( InstructionPart::ImmediateByte      ( immediate             ) );
                  }
                  else
                  {
                    return  instruction.fail
                            (
                              format!
                              (
                                "Value Out of Bonds [-0x80,0xff] {}",
                                immediate,
                              )
                            );
                  }
                }
                else
                {
                  
                }
                Ok ( None )
              }
            },
        ( OperandType::GeneralPurposeRegister ( dstRegister ),  OperandType::GeneralPurposeRegister ( srcRegister ) )
        =>  {
              Ok ( None )
            },
        ( _, _ )
        =>  {
              instruction.fail
              (
                format!
                (
                  "Invalid Combination of Arguments ›{}‹, ›{}‹",
                  instruction.operands [ 0 ].to_string(),
                  instruction.operands [ 1 ].to_string(),
                )
              )
            },
      }
    }
    else
    {
      instruction.fail
      (
        format!
        (
          "Instruction Must Take Exactly 2 Arguments, got {}",
          instruction.operands.len(),
        )
      )
    }
  }
}