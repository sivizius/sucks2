use super::
{
  expressions::
  {
    Expression,
  },
  instructions::
  {
    InstructionAddress,
  },
  memory::
  {
    Memory16Registers,
  },
  registers::
  {
    SegmentRegisterNumber,
  },
  symbols::
  {
    SymbolIdentifier,
    SymbolReference,
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
  //  symbols     are an abstract type, which cannot be encoded, but can be resolved to a reference by as hash map look up
  Symbol                                ( SymbolIdentifier      ),
  //  references  are an abstract type, which cannot be encoded, but are a reference to another operand type stored in list of symbols
  Reference                             ( SymbolReference       ),
  //  addresses   are an abstract type, which cannot be encoded, but a displacement can be calculated
  Address                               ( InstructionAddress    ),
  //  expressions are an abstract type, which cannot be encoded, but can be resolved to another operand type
  Expression                            ( Expression            ),
  //  constants   are immediate values without size, which must be obtained by context
  Constant                              ( i128                  ),
  //  relative addressing:  address +=                                                    displacement
  Displacement                          ( i128                  ),
  //  direct addressing:    address =   16  * segment                                   + displacement
  Intersegment
  {
    offset:                             i128,
    segment:                            i128,
  },
  //  indirect addressing:  address =   16  * segment register  + registers             + displacement
  Memory16
  {
    segment:                            SegmentRegisterNumber,
    registers:                          Memory16Registers,
    displacement:                       i128,
  },
  //  indirect addressing:  address =   16  * segment register  + base  + scale * index + displacement
  Memory32
  {
    segment:                            SegmentRegisterNumber,
    base:                               u8,
    scale:                              u8,
    index:                              u8,
    displacement:                       i128,
  },
  //  registers
  GeneralPurposeRegister
  {
    rex:                                bool,
    number:                             u8,
  },
  SegmentRegister                       ( SegmentRegisterNumber ),
  ControlRegister                       ( u8                    ),
  DebugRegister                         ( u8                    ),
  TestRegister                          ( u8                    ),
  MulitMediaRegister                    ( u8                    ),
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
      OperandType::Symbol                 ( name )
      =>  format! ( "${{{}}}", name ),
      OperandType::Reference              ( reference )
      =>  format! ( "$({})", reference),
      OperandType::Constant               ( constant )
      =>  format! ( "({})", constant ),
      OperandType::Displacement           ( constant )
      =>  if *constant < 0
          {
            format! ( "@-{:04x}", -constant )
          }
          else
          {
            format! ( "@+{:04x}", constant )
          },
      OperandType::Intersegment           { offset, segment }
      =>  format! ( "@{}:{}", segment, offset ),
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
      _
      =>  unimplemented!(),
    }
  }
}
