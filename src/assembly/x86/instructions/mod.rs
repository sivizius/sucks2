mod simpleMath;

pub use super::
{
  operands::
  {
    Operand,
    OperandType,
  },
};

pub struct Instruction
{
  line:                                 usize,
  mnemonic:                             &'static str,
  size:                                 usize,
  address:                              InstructionAddress,
  length:                               Option<usize>,
  prefixes:                             Vec<InstructionPrefix>,
  opcode:                               InstructionType,
  operands:                             Vec<OperandType>,
}

impl Instruction
{
  pub fn addPrefix
  (
    &mut  self,
    prefix:                             InstructionPrefix,
  )
  {
    self.prefixes.push ( prefix );
  }

  pub fn addOperand
  (
    &mut  self,
    operand:                            OperandType,
  )
  {
    self.operands.push ( operand );
  }

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

  pub fn getOperands
  (
    &self,
  ) -> Vec<OperandType>
  {
    self.operands.clone()
  }

  pub fn print
  (
    &self,
  )
  {
    print!  ( "0x{:08x} – ", self.line );
    if let InstructionAddress::Some { base, offs } = self.address
    {
      print!  ( "{:04x}:{:016x}", base, offs );
    }
    else
    {
      print!  ( "None" );
    }
    if let Some ( length ) = self.length
    {
      print!  ( "( 0x{:02x} ) ", length );
    }
    else
    {
      print!  ( "( None ) " );
    }
    print!  ( "{} ", self.mnemonic );
    match self.getOpcode()
    {
      InstructionType::Label          ( identifier  )
      =>  {
            print!  ( "({})",                   identifier  );
          },
      InstructionType::SimpleMath     ( opcode      )
      =>  {
            print!  ( "(SimpleMath: 0x{:02x})", opcode      );
          },
      InstructionType::OneByte        ( opcode )
      =>  {
            print!  ( "(OneByte:    0x{:02x})", opcode      );
          },
      _
      =>  {
            print!  ( "???"                                 );
          },
    }
    for   operand                       in  self.getOperands()
    {
      match operand
      {
        OperandType::Constant               ( constant )
        =>  print!  ( " {},", constant ),
        OperandType::Byte                   ( immediate )
        =>  print!  ( " {},", immediate ),
        OperandType::Word                   ( immediate )
        =>  print!  ( " {},", immediate ),
        OperandType::DWord                  ( immediate )
        =>  print!  ( " {},", immediate ),
        OperandType::QWord                  ( immediate )
        =>  print!  ( " {},", immediate ),
        OperandType::GeneralPurposeRegister ( register )
        =>  {
              match self.size
              {
                1 =>  {
                        match register
                        {
                           0  =>  print!  ( " al," ),
                           1  =>  print!  ( " cl," ),
                           2  =>  print!  ( " dl," ),
                           3  =>  print!  ( " bl," ),
                           //TODO haz REX?
                           4  =>  print!  ( " ah," ),
                           5  =>  print!  ( " ch," ),
                           6  =>  print!  ( " dh," ),
                           7  =>  print!  ( " bh," ),
                           8  =>  print!  ( " r8b,"  ),
                           9  =>  print!  ( " r9b,"  ),
                          10  =>  print!  ( " r10b," ),
                          11  =>  print!  ( " r11b," ),
                          12  =>  print!  ( " r12b," ),
                          13  =>  print!  ( " r13b," ),
                          14  =>  print!  ( " r14b," ),
                          15  =>  print!  ( " r15b," ),
                          _   =>  print!  ( " ???," ),
                        }
                      },
                2 =>  {
                        match register
                        {
                           0  =>  print!  ( " ax," ),
                           1  =>  print!  ( " cx," ),
                           2  =>  print!  ( " dx," ),
                           3  =>  print!  ( " bx," ),
                           4  =>  print!  ( " sp," ),
                           5  =>  print!  ( " bp," ),
                           6  =>  print!  ( " si," ),
                           7  =>  print!  ( " di," ),
                           8  =>  print!  ( " r8w,"  ),
                           9  =>  print!  ( " r9w,"  ),
                          10  =>  print!  ( " r10w," ),
                          11  =>  print!  ( " r11w," ),
                          12  =>  print!  ( " r12w," ),
                          13  =>  print!  ( " r13w," ),
                          14  =>  print!  ( " r14w," ),
                          15  =>  print!  ( " r15w," ),
                          _   =>  print!  ( " ???," ),
                        }
                      },
                4 =>  {
                        match register
                        {
                           0  =>  print!  ( " eax," ),
                           1  =>  print!  ( " ecx," ),
                           2  =>  print!  ( " edx," ),
                           3  =>  print!  ( " ebx," ),
                           4  =>  print!  ( " esp," ),
                           5  =>  print!  ( " ebp," ),
                           6  =>  print!  ( " esi," ),
                           7  =>  print!  ( " edi," ),
                           8  =>  print!  ( " r8d,"  ),
                           9  =>  print!  ( " r9d,"  ),
                          10  =>  print!  ( " r10d," ),
                          11  =>  print!  ( " r11d," ),
                          12  =>  print!  ( " r12d," ),
                          13  =>  print!  ( " r13d," ),
                          14  =>  print!  ( " r14d," ),
                          15  =>  print!  ( " r15d," ),
                          _   =>  print!  ( " ???," ),
                        }
                      },
                8 =>  {
                        match register
                        {
                           0  =>  print!  ( " rax," ),
                           1  =>  print!  ( " rcx," ),
                           2  =>  print!  ( " rdx," ),
                           3  =>  print!  ( " rbx," ),
                           4  =>  print!  ( " rsp," ),
                           5  =>  print!  ( " rbp," ),
                           6  =>  print!  ( " rsi," ),
                           7  =>  print!  ( " rdi," ),
                           8  =>  print!  ( " r8,"  ),
                           9  =>  print!  ( " r9,"  ),
                          10  =>  print!  ( " r10," ),
                          11  =>  print!  ( " r11," ),
                          12  =>  print!  ( " r12," ),
                          13  =>  print!  ( " r13," ),
                          14  =>  print!  ( " r14," ),
                          15  =>  print!  ( " r15," ),
                          _   =>  print!  ( " ???," ),
                        }
                      },
                _ =>  print!  (
                                "[{}],",
                                self.size,
                              ),
              }
            },
        _
        =>  print!  (
                      "<{}>,",
                      operand.to_string(),
                    ),
      }
    }
    println!  ( "" );
  }

  pub fn setAddress
  (
    &mut  self,
    address:                            InstructionAddress,
  )
  {
    self.address                        =   address;
  }

  pub fn setLength
  (
    &mut  self,
    length:                             Option<usize>,
  )
  {
    self.length                         =   length;
  }

  pub fn setOpcode
  (
    &mut  self,
    opcode:                             InstructionType,
  )
  {
    self.opcode                         =   opcode;
  }

  pub fn setOperands
  (
    &mut  self,
    operands:                           Vec<OperandType>,
  )
  {
    self.operands                       =   operands;
  }
}

pub fn Instruction
(
  line:                                 usize,
  mnemonic:                             &'static str,
  size:                                 usize,
  opcode:                               InstructionType,
  operands:                             Vec<OperandType>,
) -> Instruction
{
  Instruction
  {
    line:                               line,
    mnemonic:                           mnemonic,
    size:                               size,
    address:                            InstructionAddress::None,
    length:                             None,
    prefixes:                           vec!(),
    opcode:                             opcode,
    operands:                           operands,
  }
}

#[derive(Clone,Copy)]
pub enum InstructionAddress
{
  None,
  Some
  {
    base:                               usize,
    offs:                               u64,
  },
}

impl InstructionAddress
{
  pub fn add
  (
    &mut self,
    offset:                             Option<usize>,
  ) -> Result<( usize, u64 ), &'static str>
  {
    if let InstructionAddress::Some { ref mut base, ref mut offs } = self
    {
      if let Some ( diff ) = offset
      {
        let     offset                  =   *offs + diff as u64;
        *offs                           =   offset;
      }
      else
      {
        *base                           =   *base + 1;
        *offs                           =   0;
      }
      Ok
      (
        (
          *base,
          *offs,
        )
      )
    }
    else
    {
      Err ( "No Instruction Address" )
    }
  }
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

#[derive(Clone)]
pub enum InstructionType
{
  Label                                 ( usize ),
  SimpleMath                            ( u8 ),
  OneByte                               ( u8 ),
}