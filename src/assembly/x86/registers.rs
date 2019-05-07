use super::
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
  pub rex:                              bool,
  pub size:                             usize,
  pub number:                           u8,
}

impl Operand                            for GeneralPurposeRegister
{
  fn this   ( self ) -> ( OperandType, usize ) { ( OperandType::GeneralPurposeRegister { rex: self.rex, number: self.number }, self.size ) }
}

macro_rules!  GeneralPurposeRegister
{
  (
    $theName:ident,
    $theSize:expr,
    $hazREX:expr,
    $theNumber:expr
  )
  =>  {
        pub const $theName:             GeneralPurposeRegister
        = GeneralPurposeRegister
          {
            rex:                        $hazREX,
            size:                       $theSize,
            number:                     $theNumber,
          };
      }
}

pub struct    SegmentRegister
{
  pub number:                           SegmentRegisterNumber,
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

#[derive(Clone,Copy,Debug,PartialEq,PartialOrd)]
pub enum      SegmentRegisterNumber
{
  CS                                    =   0x00,
  SS                                    =   0x01,
  DS                                    =   0x02,
  ES                                    =   0x03,
  FS                                    =   0x04,
  GS                                    =   0x05,
  Default                               =   0xff,
}

impl          SegmentRegisterNumber
{
  pub fn to_string
  (
    &self,
  ) ->  &'static str
  {
    match self
    {
      SegmentRegisterNumber::CS =>  "cs",
      SegmentRegisterNumber::SS =>  "ss",
      SegmentRegisterNumber::DS =>  "ds",
      SegmentRegisterNumber::ES =>  "es",
      SegmentRegisterNumber::FS =>  "fs",
      SegmentRegisterNumber::GS =>  "gs",
      _                         =>  "??",
    }
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
  pub size:                             usize,
  pub number:                           u8,
}

impl Operand                            for MulitMediaRegister
{
  fn this   ( self ) -> ( OperandType, usize ) { ( OperandType::MulitMediaRegister ( self.number ), self.size ) }
}

macro_rules! MulitMediaRegister
{
  (
    $theName:ident,
    $theSize:expr,
    $theNumber:expr
  )
  =>  {
        pub const $theName:             MulitMediaRegister
        = MulitMediaRegister
          {
            size:                       $theSize,
            number:                     $theNumber,
          };
      }
}

impl X86
{
  GeneralPurposeRegister! ( al,   1,  false,  0                         );
  GeneralPurposeRegister! ( cl,   1,  false,  1                         );
  GeneralPurposeRegister! ( dl,   1,  false,  2                         );
  GeneralPurposeRegister! ( bl,   1,  false,  3                         );
  GeneralPurposeRegister! ( ah,   1,  false,  4                         );
  GeneralPurposeRegister! ( ch,   1,  false,  5                         );
  GeneralPurposeRegister! ( dh,   1,  false,  6                         );
  GeneralPurposeRegister! ( bh,   1,  false,  7                         );
  GeneralPurposeRegister! ( spl,  1,  true,   4                         );
  GeneralPurposeRegister! ( bpl,  1,  true,   5                         );
  GeneralPurposeRegister! ( sil,  1,  true,   6                         );
  GeneralPurposeRegister! ( dil,  1,  true,   7                         );

  GeneralPurposeRegister! ( ax,   2,  false,  0                         );
  GeneralPurposeRegister! ( cx,   2,  false,  1                         );
  GeneralPurposeRegister! ( dx,   2,  false,  2                         );
  GeneralPurposeRegister! ( bx,   2,  false,  3                         );
  GeneralPurposeRegister! ( sp,   1,  false,  4                         );
  GeneralPurposeRegister! ( bp,   1,  false,  5                         );
  GeneralPurposeRegister! ( si,   1,  false,  6                         );
  GeneralPurposeRegister! ( di,   1,  false,  7                         );

  GeneralPurposeRegister! ( eax,  4,  false,  0                         );
  GeneralPurposeRegister! ( ecx,  4,  false,  1                         );
  GeneralPurposeRegister! ( edx,  4,  false,  2                         );
  GeneralPurposeRegister! ( ebx,  4,  false,  3                         );
  GeneralPurposeRegister! ( esp,  4,  false,  4                         );
  GeneralPurposeRegister! ( ebp,  4,  false,  5                         );
  GeneralPurposeRegister! ( esi,  4,  false,  6                         );
  GeneralPurposeRegister! ( edi,  4,  false,  7                         );

  GeneralPurposeRegister! ( rax,  8,  false,  0                         );
  GeneralPurposeRegister! ( rcx,  8,  false,  1                         );
  GeneralPurposeRegister! ( rdx,  8,  false,  2                         );
  GeneralPurposeRegister! ( rbx,  8,  false,  3                         );
  GeneralPurposeRegister! ( rsp,  8,  false,  4                         );
  GeneralPurposeRegister! ( rbp,  8,  false,  5                         );
  GeneralPurposeRegister! ( rsi,  8,  false,  6                         );
  GeneralPurposeRegister! ( rdi,  8,  false,  7                         );
  GeneralPurposeRegister! ( r8,   8,  false,  8                         );
  GeneralPurposeRegister! ( r9,   8,  false,  9                         );
  GeneralPurposeRegister! ( r10,  8,  false, 10                         );
  GeneralPurposeRegister! ( r11,  8,  false, 11                         );
  GeneralPurposeRegister! ( r12,  8,  false, 12                         );
  GeneralPurposeRegister! ( r13,  8,  false, 13                         );
  GeneralPurposeRegister! ( r14,  8,  false, 14                         );
  GeneralPurposeRegister! ( r15,  8,  false, 15                         );

  SegmentRegister!        ( cs,               SegmentRegisterNumber::CS );
  SegmentRegister!        ( ss,               SegmentRegisterNumber::SS );
  SegmentRegister!        ( ds,               SegmentRegisterNumber::DS );
  SegmentRegister!        ( es,               SegmentRegisterNumber::ES );
  SegmentRegister!        ( fs,               SegmentRegisterNumber::FS );
  SegmentRegister!        ( gs,               SegmentRegisterNumber::GS );

  ControlRegister!        ( cr0,              0                         );
  ControlRegister!        ( cr1,              1                         );
  ControlRegister!        ( cr2,              2                         );
  ControlRegister!        ( cr3,              3                         );
  ControlRegister!        ( cr4,              4                         );
  ControlRegister!        ( cr5,              5                         );
  ControlRegister!        ( cr6,              6                         );
  ControlRegister!        ( cr7,              7                         );

  DebugRegister!          ( dr0,              0                         );
  DebugRegister!          ( dr1,              1                         );
  DebugRegister!          ( dr2,              2                         );
  DebugRegister!          ( dr3,              3                         );
  DebugRegister!          ( dr4,              4                         );
  DebugRegister!          ( dr5,              5                         );
  DebugRegister!          ( dr6,              6                         );
  DebugRegister!          ( dr7,              7                         );

  TestRegister!           ( tr6,              6                         );
  TestRegister!           ( tr7,              7                         );

  MulitMediaRegister!     ( mm0,  8,          0                         );
  MulitMediaRegister!     ( mm1,  8,          1                         );
  MulitMediaRegister!     ( mm2,  8,          2                         );
  MulitMediaRegister!     ( mm3,  8,          3                         );
  MulitMediaRegister!     ( mm4,  8,          4                         );
  MulitMediaRegister!     ( mm5,  8,          5                         );
  MulitMediaRegister!     ( mm6,  8,          6                         );
  MulitMediaRegister!     ( mm7,  8,          7                         );

  MulitMediaRegister!     ( xmm0, 16,         0                         );
  MulitMediaRegister!     ( xmm1, 16,         1                         );
  MulitMediaRegister!     ( xmm2, 16,         2                         );
  MulitMediaRegister!     ( xmm3, 16,         3                         );
  MulitMediaRegister!     ( xmm4, 16,         4                         );
  MulitMediaRegister!     ( xmm5, 16,         5                         );
  MulitMediaRegister!     ( xmm6, 16,         6                         );
  MulitMediaRegister!     ( xmm7, 16,         7                         );

  MulitMediaRegister!     ( ymm0, 32,         0                         );
  MulitMediaRegister!     ( ymm1, 32,         1                         );
  MulitMediaRegister!     ( ymm2, 32,         2                         );
  MulitMediaRegister!     ( ymm3, 32,         3                         );
  MulitMediaRegister!     ( ymm4, 32,         4                         );
  MulitMediaRegister!     ( ymm5, 32,         5                         );
  MulitMediaRegister!     ( ymm6, 32,         6                         );
  MulitMediaRegister!     ( ymm7, 32,         7                         );

  MulitMediaRegister!     ( zmm0, 64,         0                         );
  MulitMediaRegister!     ( zmm1, 64,         1                         );
  MulitMediaRegister!     ( zmm2, 64,         2                         );
  MulitMediaRegister!     ( zmm3, 64,         3                         );
  MulitMediaRegister!     ( zmm4, 64,         4                         );
  MulitMediaRegister!     ( zmm5, 64,         5                         );
  MulitMediaRegister!     ( zmm6, 64,         6                         );
  MulitMediaRegister!     ( zmm7, 64,         7                         );
}