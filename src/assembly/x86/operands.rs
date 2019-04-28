#[derive(Clone)]
pub enum OperandType
{
  Label                                 ( &'static str ),
  //  constant might be removed, because it is just a trivial expression
  Constant                              ( i128 ),
  Expression
  {
  },
  // segment + base + scale * index + label + offset
  Memory16
  {
    segment:                            u8,
    registers:                          u8,
    offset:                             usize,
    label:                              usize,
  },
  // segment + registers + label + offset
  Memory32
  {
    segment:                            u8,
    base:                               u8,
    scale:                              u8,
    index:                              u8,
    offset:                             usize,
    label:                              usize,
  },
  GeneralPurposeRegister                ( u8 ),
  SegmentRegister                       ( u8 ),
  ControlRegister                       ( u8 ),
  DebugRegister                         ( u8 ),
  TestRegister                          ( u8 ),
  MulitMediaRegister                    ( u8 ),
}

impl OperandType
{
  pub fn print
  (
    &self,
    size:                               usize,
  )
  {
    match self
    {
      OperandType::Constant               ( constant )
      =>  print!  ( " {},", constant ),
      OperandType::GeneralPurposeRegister ( register )
      =>  {
            match size
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
                              size,
                            ),
            }
          },
      _
      =>  print!  (
                    "<{}>,",
                    self.to_string(),
                  ),
    }
  }
  pub fn to_string
  (
    &self,
  ) -> &'static str
  {
    match self
    {
      OperandType::Label                ( _ )   =>  { "Label"                     },
      OperandType::Constant             ( _ )   =>  { "Constant"                  },
      OperandType::Expression           { .. }  =>  { "Expression"                },
      OperandType::Memory16             { .. }  =>  { "Memory (16 bit)"           },
      OperandType::Memory32             { .. }  =>  { "Memory (32 bit)"           },
      OperandType::GeneralPurposeRegister
                                        ( _ )   =>  { "General Purpose Register"  },
      OperandType::SegmentRegister      ( _ )   =>  { "Segment Register"          },
      OperandType::ControlRegister      ( _ )   =>  { "Control Register"          },
      OperandType::DebugRegister        ( _ )   =>  { "Debug Register"            },
      OperandType::TestRegister         ( _ )   =>  { "Test Register"             },
      OperandType::MulitMediaRegister   ( _ )   =>  { "Multi Media Register"      },
    }
  }
}

pub trait Operand
{
  fn this   ( self ) -> ( OperandType, usize );
}

impl Operand                            for i128
{
  fn this   ( self ) -> ( OperandType, usize ) { ( OperandType::Constant ( self ), 0 ) }
}
