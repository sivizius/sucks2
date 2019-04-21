pub use super::
{
  Instruction,
  InstructionType,
  super::
  {
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
        ) -> Result<Self, &'static str>
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

  pub(in super::super) fn compileSimpleMathInstruction
  (
    instruction:                        Instruction,
    opcode:                             u8,
  ) -> Result<usize, String>
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
                          Ok ( 1 )
                        }
                        else
                        {
                          Err
                          (
                            format!
                            (
                              "Value Out of Bonds [-0x80,0xff] {}",
                              *immediate,
                            )
                          )
                        }
                      },
                  2
                  =>  {
                        if  *immediate >= -0x8000
                        &&  *immediate <=  0xffff
                        {
                          Ok ( 1 )
                        }
                        else
                        {
                          Err
                          (
                            format!
                            (
                              "Value Out of Bonds [-0x8000,0xffff] {}",
                              *immediate,
                            )
                          )
                        }
                      },
                  4 //if operandSize > 16
                  =>  {
                        if  *immediate >= -0x80000000
                        &&  *immediate <=  0xffffffff
                        {
                          Ok ( 1 )
                        }
                        else
                        {
                          Err
                          (
                            format!
                            (
                              "Value Out of Bonds [-0x80000000,0xffffffff] {}",
                              *immediate,
                            )
                          )
                        }
                      },
                  _
                  =>  {
                        Err
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
                Ok ( 3 )
              }
            },
        ( OperandType::GeneralPurposeRegister ( dstRegister ),  OperandType::GeneralPurposeRegister ( srcRegister ) )
        =>  {
              Ok ( 2 )
            },
        ( _, _ )
        =>  {
              Err
              (
                format!
                (
                  "Invalid Combination of Arguments ›{}‹, ›{}‹ for Instruction ›{}‹",
                  instruction.operands [ 0 ].to_string(),
                  instruction.operands [ 1 ].to_string(),
                  instruction.mnemonic,
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
          "Instruction ›{}‹ Must Take Exactly 2 Arguments",
          instruction.mnemonic
        )
      )
    }
  }
}