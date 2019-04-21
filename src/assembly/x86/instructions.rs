pub use super::
{
  operands::
  {
    Operand,
    OperandType
  },
};

pub enum InstructionType
{
  Label                                 ( usize ),
  SimpleMath                            ( u8 ),
  OneByte                               ( u8 ),
}

pub struct Instruction
{
  pub line:                             usize,
  pub mnemonic:                         &'static str,
  pub size:                             usize,
  pub length:                           Option<usize>,
  pub opcode:                           InstructionType,
  pub operands:                         Vec<OperandType>,
}

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
