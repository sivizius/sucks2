use super::
{
  operands::
  {
    Operand,
    OperandType
  },
  registers::
  {
    SegmentRegisterNumber,
  },
};

pub struct Memory16
{
  size:                                 usize,
  segment:                              SegmentRegisterNumber,
  registers:                            Memory16Registers,
  displacement:                         i128,
}

pub fn Memory16
(
  size:                                 usize,
  segment:                              SegmentRegisterNumber,
  registers:                            Memory16Registers,
  displacement:                         i128,
) ->  Memory16
{
  Memory16
  {
    size:                               size,
    segment:                            segment,
    registers:                          registers,
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

#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
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
  INVALID                               =   0xff,
}

#[macro_export]
macro_rules! x86Mem16finally
{
  ( $size:expr, $segment:expr, $(  $token:tt )* )
  =>  {
        Expression
        (
          vec!
          [
            $(
              nextToken!
              (
                $token
              ),
            )*
            ExpressionToken::Memory16
            {
              size:                     $size,
              segment:                  $segment,
              registers:                Memory16Registers::INVALID,
              displacement:             0,
            }
          ]
        )
      };
}

#[macro_export]
macro_rules! x86Mem16segment
{
  ( $size:expr, cs, $(  $token:tt )* )  =>  { x86Mem16finally!  ( $size,  SegmentRegisterNumber::CS,      $( $token )*  ) };
  ( $size:expr, ss, $(  $token:tt )* )  =>  { x86Mem16finally!  ( $size,  SegmentRegisterNumber::SS,      $( $token )*  ) };
  ( $size:expr, ds, $(  $token:tt )* )  =>  { x86Mem16finally!  ( $size,  SegmentRegisterNumber::DS,      $( $token )*  ) };
  ( $size:expr, es, $(  $token:tt )* )  =>  { x86Mem16finally!  ( $size,  SegmentRegisterNumber::ES,      $( $token )*  ) };
  ( $size:expr, fs, $(  $token:tt )* )  =>  { x86Mem16finally!  ( $size,  SegmentRegisterNumber::FS,      $( $token )*  ) };
  ( $size:expr, gs, $(  $token:tt )* )  =>  { x86Mem16finally!  ( $size,  SegmentRegisterNumber::GS,      $( $token )*  ) };
  ( $size:expr, @,  $(  $token:tt )* )  =>  { x86Mem16finally!  ( $size,  SegmentRegisterNumber::Default, $( $token )*  ) };
}

#[macro_export]
macro_rules! x86Mem16
{
  ( byte                      [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 1,      @,      $( $token )* ) };
  ( byte          $sreg:tt  : [ $(  $token:tt )*  ] ) =>  { x86Mem16segment!  ( 1,      $sreg,  $( $token )* ) };
  ( word                      [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 2,      @,      $( $token )* ) };
  ( word          $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 2,      $sreg,  $( $token )* ) };
  ( dword                     [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 4,      @,      $( $token )* ) };
  ( dword         $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 4,      $sreg,  $( $token )* ) };
  ( qword                     [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 8,      @,      $( $token )* ) };
  ( qword         $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( 8,      $sreg,  $( $token )* ) };
  ( $size:literal             [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( $size,  @,      $( $token )* ) };
  ( $size:literal $sreg:tt  : [ $(  $token:tt )+  ] ) =>  { x86Mem16segment!  ( $size,  $sreg,  $( $token )* ) };
}