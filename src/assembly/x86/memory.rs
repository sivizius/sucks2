use super::
{
  expressions::
  {
    Expression,
    ExpressionToken,
  },
  operands::
  {
    Operand,
    OperandType
  },
};

pub struct Memory16
{
  size:                                 usize,
  segment:                              u8,
  registers:                            u8,
  displacement:                         i128,
}

pub fn Memory16
(
  size:                                 usize,
  segment:                              u8,
  registers:                            Memory16Registers,
  displacement:                         i128,
) ->  Memory16
{
  Memory16
  {
    size:                               size,
    segment:                            segment,
    registers:                          registers as u8,
    displacement:                       displacement,
  }
}

impl Operand                            for Memory16
{
  fn this
  (
    self
  ) ->  ( OperandType, usize )
  {
    (
      OperandType::Memory16
      {
        segment:                        self.segment,
        registers:                      self.registers,
        displacement:                   self.displacement,
      },
      self.size,
    )
  }
}

pub enum Memory16Registers
{
  BXSI                                  =   0x00,
  BXDI                                  =   0x01,
  BPSI                                  =   0x02,
  BPDI                                  =   0x03,
  SI                                    =   0x04,
  DI                                    =   0x05,
  BP                                    =   0x06,
  BX                                    =   0x07,
  DISP                                  =   0x86,
}

#[macro_export]
macro_rules! x86Mem16segment
{
  ( $size:expr, cs, $(  $token:tt )* )  =>  { Expression  ( vec!  [ $(  nextToken!  ( $token  ),  )*  ExpressionToken::Memory16 { size: $size,  segment:  0 } ] ) };
  ( $size:expr, ss, $(  $token:tt )* )  =>  { Expression  ( vec!  [ $(  nextToken!  ( $token  ),  )*  ExpressionToken::Memory16 { size: $size,  segment:  1 } ] ) };
  ( $size:expr, ds, $(  $token:tt )* )  =>  { Expression  ( vec!  [ $(  nextToken!  ( $token  ),  )*  ExpressionToken::Memory16 { size: $size,  segment:  2 } ] ) };
  ( $size:expr, es, $(  $token:tt )* )  =>  { Expression  ( vec!  [ $(  nextToken!  ( $token  ),  )*  ExpressionToken::Memory16 { size: $size,  segment:  3 } ] ) };
  ( $size:expr, fs, $(  $token:tt )* )  =>  { Expression  ( vec!  [ $(  nextToken!  ( $token  ),  )*  ExpressionToken::Memory16 { size: $size,  segment:  4 } ] ) };
  ( $size:expr, gs, $(  $token:tt )* )  =>  { Expression  ( vec!  [ $(  nextToken!  ( $token  ),  )*  ExpressionToken::Memory16 { size: $size,  segment:  5 } ] ) };
}

#[macro_export]
macro_rules! x86Mem16
{
  ( byte                      [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 1,      ds,     $( $token )* ) };
  ( byte          $sreg:tt  : [ $(  $token:tt )*  ] ) =>  { x86Mem16segment!  ( 1,      $sreg,  $( $token )* ) };
  ( word                      [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 2,      ds,     $( $token )* ) };
  ( word          $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 2,      $sreg,  $( $token )* ) };
  ( dword                     [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 4,      ds,     $( $token )* ) };
  ( dword         $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 4,      $sreg,  $( $token )* ) };
  ( qword                     [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 8,      ds,     $( $token )* ) };
  ( qword         $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 8,      $sreg,  $( $token )* ) };
  ( $size:literal             [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( $size,  ds,     $( $token )* ) };
  ( $size:literal $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( $size,  $sreg,  $( $token )* ) };
}