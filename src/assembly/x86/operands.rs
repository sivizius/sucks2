#[derive(Clone)]
pub enum OperandType
{
  //  label and constant might be removed,
  //  because it is just a trivial case of expressions
  Label                                 ( &'static str ),
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
    hazREX:                             bool,
  )
  {
    match self
    {
      OperandType::Label                  ( name )
      =>  print!  ( " @{},", name ),
      OperandType::Constant               ( constant )
      =>  print!  ( " {},", constant ),
      OperandType::Expression             {}
      =>  print!  ( " ()," ),
      OperandType::Memory16               { .. }
      =>  print!  ( " […]," ),
      OperandType::Memory32               { .. }
      =>  print!  ( " […]," ),
      OperandType::GeneralPurposeRegister ( register )
      =>  {
            match size
            {
              1 =>  {
                      match register
                      {
                        0 =>  print!  ( " al,"                        ),
                        1 =>  print!  ( " cl,"                        ),
                        2 =>  print!  ( " dl,"                        ),
                        3 =>  print!  ( " bl,"                        ),
                        4 if hazREX
                        =>    print!  ( " ah,"                        ),
                        4 =>  print!  ( " spl,"                       ),
                        5 if hazREX
                        =>    print!  ( " ch,"                        ),
                        5 =>  print!  ( " bpl,"                       ),
                        6 if hazREX
                        =>    print!  ( " dh,"                        ),
                        6 =>  print!  ( " sil,"                       ),
                        7 if hazREX
                        =>    print!  ( " bh,"                        ),
                        7 =>  print!  ( " dil,"                       ),
                        8 ... 15
                        =>    print!  ( " r{}b,",           register, ),
                        _
                        =>    print!  ( " r{}b?,",          register, ),
                      }
                    },
              2 =>  {
                      match register
                      {
                        0 =>  print!  ( " ax,"                        ),
                        1 =>  print!  ( " cx,"                        ),
                        2 =>  print!  ( " dx,"                        ),
                        3 =>  print!  ( " bx,"                        ),
                        4 =>  print!  ( " sp,"                        ),
                        5 =>  print!  ( " bp,"                        ),
                        6 =>  print!  ( " si,"                        ),
                        7 =>  print!  ( " di,"                        ),
                        8 ... 15
                        =>    print!  ( " r{}w,",           register, ),
                        _
                        =>    print!  ( " r{}w?,",          register, ),
                      }
                    },
              4 =>  {
                      match register
                      {
                        0 =>  print!  ( " eax,"                       ),
                        1 =>  print!  ( " ecx,"                       ),
                        2 =>  print!  ( " edx,"                       ),
                        3 =>  print!  ( " ebx,"                       ),
                        4 =>  print!  ( " esp,"                       ),
                        5 =>  print!  ( " ebp,"                       ),
                        6 =>  print!  ( " esi,"                       ),
                        7 =>  print!  ( " edi,"                       ),
                        8 ... 15
                        =>    print!  ( " r{}d,",           register, ),
                        _
                        =>    print!  ( " r{}d?,",          register, ),
                      }
                    },
              8 =>  {
                      match register
                      {
                        0 =>  print!  ( " rax,"                       ),
                        1 =>  print!  ( " rcx,"                       ),
                        2 =>  print!  ( " rdx,"                       ),
                        3 =>  print!  ( " rbx,"                       ),
                        4 =>  print!  ( " rsp,"                       ),
                        5 =>  print!  ( " rbp,"                       ),
                        6 =>  print!  ( " rsi,"                       ),
                        7 =>  print!  ( " rdi,"                       ),
                        8 ... 15
                        =>    print!  ( " r{},",            register, ),
                        _
                        =>    print!  ( " r{}?,",           register, ),
                      }
                    },
              _ =>            print!  ( "({})r{}?,",  size, register, ),
            }
          },
      OperandType::SegmentRegister      ( register )
      =>  {
            match register
            {
              0 =>  print!  ( " cs,"              ),
              1 =>  print!  ( " ss,"              ),
              2 =>  print!  ( " ds,"              ),
              3 =>  print!  ( " es,"              ),
              4 =>  print!  ( " fs,"              ),
              5 =>  print!  ( " gs,"              ),
              _ =>  print!  ( " {}s,",  register  ),
            }
          },
      OperandType::ControlRegister      ( register )
      =>  print!  ( " cr{},", register ),
      OperandType::DebugRegister        ( register )
      =>  print!  ( " dr{},", register ),
      OperandType::TestRegister         ( register )
      =>  print!  ( " dr{},", register ),
      OperandType::MulitMediaRegister   ( register )
      =>  {
            match size
            {
               8  =>  print!  ( " mm{},",             register ),
              16  =>  print!  ( " xmm{},",            register ),
              32  =>  print!  ( " ymm{},",            register ),
              64  =>  print!  ( " zmm{},",            register ),
              _   =>  print!  ( " ({})mm{}?,",  size, register ),
            }
          },
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
