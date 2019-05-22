use super::
{
  Instruction,
  InstructionAddress,
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
    $theInstruction:expr,
    $theFeatures:expr
  )
  =>  {
        pub fn $theName
        (
          self,
          dst:                          impl Operand,
          src:                          impl Operand,
        ) -> Self
        {
          self.simpleMathInstruction  ( $theInstruction,  $theFeatures, dst,  src )
        }
      }
}

impl X86
{
  declareSimpleMathInstruction! ( add,  InstructionType::ADD, AssemblyFeatures::X86SignExtensionAllowed );
  declareSimpleMathInstruction! ( or,   InstructionType::OR,  AssemblyFeatures::None                    );
  declareSimpleMathInstruction! ( adc,  InstructionType::ADC, AssemblyFeatures::X86SignExtensionAllowed );
  declareSimpleMathInstruction! ( sbb,  InstructionType::SBB, AssemblyFeatures::X86SignExtensionAllowed );
  declareSimpleMathInstruction! ( and,  InstructionType::AND, AssemblyFeatures::None                    );
  declareSimpleMathInstruction! ( sub,  InstructionType::SUB, AssemblyFeatures::X86SignExtensionAllowed );
  declareSimpleMathInstruction! ( xor,  InstructionType::XOR, AssemblyFeatures::None                    );
  declareSimpleMathInstruction! ( cmp,  InstructionType::CMP, AssemblyFeatures::X86SignExtensionAllowed );

  fn simpleMathInstruction
  (
    mut self,
    instruction:                        InstructionType,
    features:                           AssemblyFeatures,
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
        self.features | features,
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
    opcode:                             u8,
  ) -> Result<Option<usize>, String>
  {
    if instruction.operands.len() == 2
    {
      match ( &instruction.operands [ 0 ], &instruction.operands [ 1 ] )
      {
        (
          OperandType::GeneralPurposeRegister { rex:      dstREX,       number: mut dstRegister                                     },
          OperandType::Constant               (           mut immediate                                                             )
        )
        =>  if  ( dstRegister == 0 )
            && !( instruction.features.hazFeature ( AssemblyFeatures::RandomOpcodeSize ) && rand::random() )
            {
              instruction.setImmediate ( instruction.size, immediate );
              match instruction.size
              {
                1
                =>  {
                      if  immediate >= -0x80
                      &&  immediate <=  0xff
                      {
                        instruction.setOpcode                 ( opcode  | 4 );
                        Ok    ( Some  ( 2 ) )
                      }
                      else
                      {
                        instruction.failOutOfBounds
                        (
                          -0x80,
                          0xff,
                          immediate,
                        )
                      }
                    },
                2
                =>  {
                      if  immediate >= -0x8000
                      &&  immediate <=  0xffff
                      {
                        instruction.setOpcode                 ( opcode  | 5 );
                        if operandSize == 4
                        {
                          instruction.setOperandSizeOverride  ( true        );
                          Ok  ( Some  ( 4 ) )
                        }
                        else
                        {
                          Ok  ( Some  ( 3 ) )
                        }
                      }
                      else
                      {
                        instruction.failOutOfBounds
                        (
                          -0x8000,
                          0xffff,
                          immediate,
                        )
                      }
                    },
                4 if architecture >= InstructionSet::i386
                =>  {
                      if  immediate >= -0x80000000
                      &&  immediate <=  0xffffffff
                      {
                        instruction.setOpcode                 ( opcode  | 5 );
                        if operandSize == 2
                        {
                          instruction.setOperandSizeOverride  ( true        );
                          Ok  ( Some  ( 6 ) )
                        }
                        else
                        {
                          Ok  ( Some  ( 5 ) )
                        }
                      }
                      else
                      {
                        instruction.failOutOfBounds
                        (
                          -0x80000000,
                          0xffffffff,
                          immediate,
                        )
                      }
                    },
                _   =>  instruction.failOperandSize (),
              }
            }
            else
            {
              instruction.encodeModRegRMdata
              (
                architecture,
                operandSize,
                if  instruction.size  == 1
                &&  architecture      < InstructionSet::amd64
                &&  instruction.features.hazFeature ( AssemblyFeatures::RandomOpcode )
                &&  rand::random()
                {
                  //  0x80 and 0x82 are aliases, but 0x82 is invalid for 64 bit.
                  //  because 0x80 is the default encoding, some disassemblers fail with 0x82.
                  0x82
                }
                else
                {
                  0x80
                },
                1,
                0,
                dstRegister | opcode,
                None,
                Some  ( immediate ),
              )
            },
        (
          OperandType::Memory16               { segment:  dstSegment,   registers:      dstRegisters, displacement: dstDisplacement },
          OperandType::Constant               (           mut immediate                                                             )
        )
        =>  instruction.encodeModRegRMdata
            (
              architecture,
              operandSize,
              if  instruction.size  == 1
              &&  architecture      < InstructionSet::amd64
              &&  instruction.features.hazFeature ( AssemblyFeatures::RandomOpcode )
              &&  rand::random()
              {
                //  0x80 and 0x82 are aliases, but 0x82 is invalid for 64 bit.
                //  because 0x80 is the default encoding, some disassemblers fail with 0x82.
                0x82
              }
              else
              {
                0x80
              },
              1,
              0,
              *dstRegisters as  u8  | opcode,
              Some  ( *dstDisplacement  ),
              Some  ( immediate ),
            ),
        (
          OperandType::GeneralPurposeRegister { rex:      dstREX,       number: mut dstRegister                                     },
          OperandType::GeneralPurposeRegister { rex:      srcREX,       number: mut srcRegister                                     }
        )
        =>  if  instruction.features.hazFeature ( AssemblyFeatures::RandomOpcode )
            &&  rand::random()
            {
              instruction.encodeModRegRMdata
              (
                architecture,
                operandSize,
                opcode | 2,
                1,
                dstRegister,
                srcRegister,
                None,
                None,
              )
            }
            else
            {
              instruction.encodeModRegRMdata
              (
                architecture,
                operandSize,
                opcode | 0,
                1,
                srcRegister <<  3,
                dstRegister,
                None,
                None,
              )
            },
        (
          OperandType::GeneralPurposeRegister { rex:      dstREX,       number:     mut dstRegister                                 },
          OperandType::Memory16               { segment:  srcSegment,   registers:      srcRegisters, displacement: srcDisplacement }
        )
        =>  instruction.encodeModRegRMdata
            (
              architecture,
              operandSize,
              opcode | 2,
              1,
              dstRegister,
              *srcRegisters as  u8,
              Some  ( *srcDisplacement  ),
              None,
            ),
        (
          OperandType::Memory16               { segment:  dstSegment,     registers:      dstRegisters, displacement: dstDisplacement },
          OperandType::GeneralPurposeRegister { rex:      srcREX,         number:     mut srcRegister                                 }
        )
        =>  instruction.encodeModRegRMdata
            (
              architecture,
              operandSize,
              opcode | 0,
              1,
              srcRegister,
              *dstRegisters as  u8,
              Some  ( *dstDisplacement  ),
              None,
            ),
        ( _, _ )
        =>  {
              instruction.fail
              (
                format!
                (
                  "Invalid Combination of Arguments ›{}‹, ›{}‹",
                  instruction.operands [ 0 ].to_string  ( instruction.size  ),
                  instruction.operands [ 1 ].to_string  ( instruction.size  ),
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