// segment + registers + label + offset
pub struct OperandMemory16
{
  segment:                              u8,
  registers:                            u8,
  offset:                               usize,
  label:                                usize,
}

// segment + base + scale * index + label + offset
pub struct OperandMemory32
{
  segment:                              u8,
  base:                                 u8,
  scale:                                u8,
  index:                                u8,
  offset:                               usize,
  label:                                usize,
}

pub struct OperandExpression
{
  
}

pub enum OperandType
{
  Label,
  Constant                              ( i128 ),
  Expression                            ( OperandExpression ),
  Memory16                              ( OperandMemory16   ),
  Memory32                              ( OperandMemory32   ),
  GeneralPurposeRegister                ( u8 ),
  SegmentRegister                       ( u8 ),
  ControlRegister                       ( u8 ),
  DebugRegister                         ( u8 ),
  TestRegister                          ( u8 ),
  MulitMediaRegister                    ( u8 ),
  Byte                                  ( i128 ),
  Word                                  ( i128 ),
  DWord                                 ( i128 ),
  QWord                                 ( i128 ),
}

impl OperandType
{
  pub fn to_string
  (
    &self,
  ) -> &'static str
  {
    match self
    {
      OperandType::Label                      =>  { "Label"                     },
      OperandType::Constant             ( _ ) =>  { "Constant"                  },
      OperandType::Expression           ( _ ) =>  { "Expression"                },
      OperandType::Memory16             ( _ ) =>  { "Memory (16 bit)"           },
      OperandType::Memory32             ( _ ) =>  { "Memory (32 bit)"           },
      OperandType::GeneralPurposeRegister
                                        ( _ ) =>  { "General Purpose Register"  },
      OperandType::SegmentRegister      ( _ ) =>  { "Segment Register"          },
      OperandType::ControlRegister      ( _ ) =>  { "Control Register"          },
      OperandType::DebugRegister        ( _ ) =>  { "Debug Register"            },
      OperandType::TestRegister         ( _ ) =>  { "Test Register"             },
      OperandType::MulitMediaRegister   ( _ ) =>  { "Multi Media Register"      },
      OperandType::Byte                 ( _ ) =>  { "Byte"                      },
      OperandType::Word                 ( _ ) =>  { "Word"                      },
      OperandType::DWord                ( _ ) =>  { "DWord"                     },
      OperandType::QWord                ( _ ) =>  { "QWord"                     },
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

