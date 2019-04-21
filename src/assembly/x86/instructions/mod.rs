mod simpleMath;

pub use super::
{
  operands::
  {
    Operand,
    OperandType,
  },
};

#[derive(Clone)]
pub enum InstructionType
{
  Label                                 ( usize ),
  SimpleMath                            ( u8 ),
  OneByte                               ( u8 ),
}

#[allow(non_camel_case_types)]
pub enum InstructionPrefix
{
  ES                                    =   0x26,
  CS                                    =   0x2e,
  SS                                    =   0x36,
  DS                                    =   0x3e,
  REX____                               =   0x40,
  REX___B                               =   0x41,
  REX__X_                               =   0x42,
  REX__XB                               =   0x43,
  REX_R__                               =   0x44,
  REX_R_B                               =   0x45,
  REX_RX_                               =   0x46,
  REX_RXB                               =   0x47,
  REXW___                               =   0x48,
  REXW__B                               =   0x49,
  REXW_X_                               =   0x4a,
  REXW_XB                               =   0x4b,
  REXWR__                               =   0x4c,
  REXWR_B                               =   0x4d,
  REXWRX_                               =   0x4e,
  REXWRXB                               =   0x4f,
  FS                                    =   0x64,
  GS                                    =   0x65,
  OperandSizeOverride                   =   0x66,
  AddressSizeOverride                   =   0x67,
  ThreeByteXOP                          =   0x8f,
  ThreeByteVEX                          =   0xc4,
  TwoByteVEX                            =   0xc5,
  LOCK                                  =   0xf0,
  REPN                                  =   0xf2,
  REP                                   =   0xf3,
}

impl InstructionPrefix
{
  pub const BranchNotTaken: Self        =   InstructionPrefix::CS;
  pub const BranchTaken:    Self        =   InstructionPrefix::DS;
}

pub struct Instruction
{
  line:                                 usize,
  mnemonic:                             &'static str,
  size:                                 usize,
  length:                               Option<usize>,
  prefixes:                             Vec<InstructionPrefix>,
  opcode:                               InstructionType,
  operands:                             Vec<OperandType>,
}

impl Instruction
{
  pub fn getLineNumber
  (
    &self,
  ) -> usize
  {
    self.line
  }

  pub fn getMnemonic
  (
    &self,
  ) -> &'static str
  {
    self.mnemonic
  }

  pub fn getOpcode
  (
    &self,
  ) -> InstructionType
  {
    self.opcode.clone()
  }

  pub fn setOpcode
  (
    &mut  self,
    opcode:                             InstructionType,
  )
  {
    self.opcode                         =   opcode;
  }

  pub fn addPrefix
  (
    &mut  self,
    prefix:                             InstructionPrefix,
  )
  {
    self.prefixes.push ( prefix );
  }
}

pub fn Instruction
(
  line:                                 usize,
  mnemonic:                             &'static str,
  size:                                 usize,
  length:                               Option<usize>,
  opcode:                               InstructionType,
  operands:                             Vec<OperandType>,
) -> Instruction
{
  Instruction
  {
    line:                               line,
    mnemonic:                           mnemonic,
    size:                               size,
    length:                             length,
    prefixes:                           vec!(),
    opcode:                             opcode,
    operands:                           operands,
  }
}
