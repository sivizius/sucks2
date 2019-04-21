pub use super::
{
  operands::
  {
    Operand,
    OperandType
  },
};


pub struct    GeneralPurposeRegister
{
  pub size:                             usize,
  pub number:                           u8,
}

impl Operand                            for GeneralPurposeRegister
{
  fn this   ( self ) -> ( OperandType, usize ) { ( OperandType::GeneralPurposeRegister ( self.number ), self.size ) }
}

macro_rules!  GeneralPurposeRegister
{
  (
    $theName:ident,
    $theSize:expr,
    $theNumber:expr
  )
  =>  {
        pub const $theName:             GeneralPurposeRegister
        = GeneralPurposeRegister
          {
            size:                       $theSize,
            number:                     $theNumber,
          };
      }
}

pub struct    SegmentRegister
{
  pub number:                           u8,
}

impl Operand                            for SegmentRegister
{
  fn this   ( self ) -> ( OperandType, usize ) { ( OperandType::SegmentRegister ( self.number ), 4 ) }
}

macro_rules!  SegmentRegister
{
  (
    $theName:ident,
    $theNumber:expr
  )
  => {
        pub const $theName:             SegmentRegister
        = SegmentRegister
          {
            number:                     $theNumber,
          };
      }
}

pub struct ControlRegister
{
  pub number:                           u8,
}

impl Operand                            for ControlRegister
{
  fn this   ( self ) -> ( OperandType, usize ) { ( OperandType::ControlRegister ( self.number ), 4 ) }
}

macro_rules!  ControlRegister
{
  (
    $theName:ident,
    $theNumber:expr
  )
  =>  {
        pub const $theName:             ControlRegister
        = ControlRegister
          {
            number:                     $theNumber,
          };
      }
}

pub struct DebugRegister
{
  pub number:                           u8,
}

impl Operand                            for DebugRegister
{
  fn this   ( self ) -> ( OperandType, usize ) { ( OperandType::ControlRegister ( self.number ), 4 ) }
}

macro_rules!  DebugRegister
{
  (
    $theName:ident,
    $theNumber:expr
  )
  =>  {
        pub const $theName:             DebugRegister
        = DebugRegister
          {
            number:                     $theNumber,
          };
      }
}

pub struct TestRegister
{
  pub number:                           u8,
}

impl Operand                            for TestRegister
{
  fn this   ( self ) -> ( OperandType, usize ) { ( OperandType::TestRegister ( self.number ), 4 ) }
}

macro_rules!  TestRegister
{
  (
    $theName:ident,
    $theNumber:expr
  )
  =>  {
        pub const $theName:             TestRegister
        = TestRegister
          {
            number:                     $theNumber,
          };
      }
}

pub struct MulitMediaRegister
{
  pub number:                           u8,
}

impl Operand                            for MulitMediaRegister
{
  fn this   ( self ) -> ( OperandType, usize ) { ( OperandType::MulitMediaRegister ( self.number ), 4 ) }
}

macro_rules! MulitMediaRegister
{
  (
    $theName:ident,
    $theNumber:expr
  )
  =>  {
        pub const $theName:             MulitMediaRegister
        = MulitMediaRegister
          {
            number:                     $theNumber,
          };
      }
}
