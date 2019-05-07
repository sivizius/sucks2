use super::
{
  expressions::
  {
    Expression,
    ExpressionToken,
  },
  memory::
  {
    Memory16Registers,
  },
  registers::
  {
    SegmentRegisterNumber,
  },
};

pub trait Operand
{
  fn this   ( self ) -> ( OperandType, usize );
}

impl Operand                            for i128
{
  fn this   ( self ) -> ( OperandType, usize ) { ( OperandType::Constant ( self ), 0 ) }
}

#[derive(Clone)]
pub enum OperandType
{
  //  label might be removed, because it is just a abstract constant
  Label                                 ( &'static str ),

  //  expressions cannot be used directly and have to be resolved to a less abstract operand type
  Expression                            ( Expression ),
  Constant                              ( i128 ),
  // segment + base + scale * index + label + offset
  Memory16
  {
    segment:                            SegmentRegisterNumber,
    registers:                          Memory16Registers,
    displacement:                       i128,
  },
  // segment + registers + label + offset
  Memory32
  {
    segment:                            SegmentRegisterNumber,
    base:                               u8,
    scale:                              u8,
    index:                              u8,
    displacement:                       i128,
  },
  GeneralPurposeRegister
  {
    rex:                                bool,
    number:                             u8,
  },
  SegmentRegister                       ( SegmentRegisterNumber ),
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
    print!  ( " {},", self.to_string  ( size ) );
  }
  pub fn to_string
  (
    &self,
    size:                               usize,
  ) ->  String
  {
    match self
    {
      OperandType::Label                  ( name )
      =>  format! ( "@{}", name ),
      OperandType::Constant               ( constant )
      =>  format! ( "{}", constant ),
      OperandType::Expression             ( expression )
      =>  format! ( "{:?}", expression ),
      OperandType::Memory16               { segment, registers, displacement }
      =>  format!
          (
            "{} {}:[ {}{} ]",
            match size
            {
              1 =>  "byte".to_string(),
              2 =>  "word".to_string(),
              4 =>  "dword".to_string(),
              8 =>  "qword".to_string(),
              _ =>  format! ( "{}", size ),
            },
            segment.to_string(),
            displacement,
            match registers
            {
              Memory16Registers::BXSI   =>  " + bx + si",
              Memory16Registers::BXDI   =>  " + bx + di",
              Memory16Registers::BPSI   =>  " + bp + si",
              Memory16Registers::BPDI   =>  " + bp + di",
              Memory16Registers::SI     =>  " + si",
              Memory16Registers::DI     =>  " + di",
              Memory16Registers::BP     =>  " + bp",
              Memory16Registers::BX     =>  " + bx",
              Memory16Registers::DISP   =>  "",
              _                         =>  " + ???",
            },
          ),
      OperandType::Memory32               { .. }
      =>  format! ( "[…]" ),
      OperandType::GeneralPurposeRegister { rex, number }
      =>  {
            match size
            {
              1 =>  {
                      match *number
                      {
                        0 =>  format! ( "al"                      ),
                        1 =>  format! ( "cl"                      ),
                        2 =>  format! ( "dl"                      ),
                        3 =>  format! ( "bl"                      ),
                        4 if *rex
                        =>    format! ( "ah"                      ),
                        4 =>  format! ( "spl"                     ),
                        5 if *rex
                        =>    format! ( "ch"                      ),
                        5 =>  format! ( "bpl"                     ),
                        6 if *rex
                        =>    format! ( "dh"                      ),
                        6 =>  format! ( "sil"                     ),
                        7 if *rex
                        =>    format! ( "bh"                      ),
                        7 =>  format! ( "dil"                     ),
                        8 ... 15
                        =>    format! ( "r{}b",           *number ),
                        _
                        =>    format! ( "r{}b?",          *number ),
                      }
                    },
              2 =>  {
                      match *number
                      {
                        0 =>  format! ( "ax"                      ),
                        1 =>  format! ( "cx"                      ),
                        2 =>  format! ( "dx"                      ),
                        3 =>  format! ( "bx"                      ),
                        4 =>  format! ( "sp"                      ),
                        5 =>  format! ( "bp"                      ),
                        6 =>  format! ( "si"                      ),
                        7 =>  format! ( "di"                      ),
                        8 ... 15
                        =>    format! ( "r{}w",           *number ),
                        _
                        =>    format! ( "r{}w?",          *number ),
                      }
                    },
              4 =>  {
                      match *number
                      {
                        0 =>  format! ( "eax"                     ),
                        1 =>  format! ( "ecx"                     ),
                        2 =>  format! ( "edx"                     ),
                        3 =>  format! ( "ebx"                     ),
                        4 =>  format! ( "esp"                     ),
                        5 =>  format! ( "ebp"                     ),
                        6 =>  format! ( "esi"                     ),
                        7 =>  format! ( "edi"                     ),
                        8 ... 15
                        =>    format! ( "r{}d",           *number ),
                        _
                        =>    format! ( "r{}d?",          *number ),
                      }
                    },
              8 =>  {
                      match *number
                      {
                        0 =>  format! ( "rax"                     ),
                        1 =>  format! ( "rcx"                     ),
                        2 =>  format! ( "rdx"                     ),
                        3 =>  format! ( "rbx"                     ),
                        4 =>  format! ( "rsp"                     ),
                        5 =>  format! ( "rbp"                     ),
                        6 =>  format! ( "rsi"                     ),
                        7 =>  format! ( "rdi"                     ),
                        8 ... 15
                        =>    format! ( "r{}",            *number ),
                        _
                        =>    format! ( "r{}?",           *number ),
                      }
                    },
              _ =>            format! ( "({})r{}?",  size, *number ),
            }
          },
      OperandType::SegmentRegister      ( register )
      =>  register.to_string().to_string(),
      OperandType::ControlRegister      ( register )
      =>  format! ( "cr{}", register ),
      OperandType::DebugRegister        ( register )
      =>  format! ( "dr{}", register ),
      OperandType::TestRegister         ( register )
      =>  format! ( "dr{}", register ),
      OperandType::MulitMediaRegister   ( register )
      =>  {
            match size
            {
               8  =>  format! ( "mm{}",             register  ),
              16  =>  format! ( "xmm{}",            register  ),
              32  =>  format! ( "ymm{}",            register  ),
              64  =>  format! ( "zmm{}",            register  ),
              _   =>  format! ( "({})mm{}?",  size, register  ),
            }
          },
    }
  }
}
