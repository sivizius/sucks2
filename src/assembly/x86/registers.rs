pub use super::
{
  X86,
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
}