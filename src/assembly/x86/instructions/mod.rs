mod simpleMath;
mod zeroOperands;

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
  //  for debugging, writable once
  line:                                 usize,
  //  for input, writeable once or after solving expressions
  size:                                 usize,
  instruction:                          InstructionType,
  operands:                             Vec<OperandType>,
  //  for processing, initialised empty/invalid
  address:                              InstructionAddress,
  hazLock:                              bool,
  theRepeat:                            u8,
  theSegmentOverride:                   u8,
  theBranchHint:                        u8,
  hazOperandSizeOverride:               bool,
  hazAddressSizeOverride:               bool,
  hazThreeByteXOP:                      bool,
  hazTwoByteVEX:                        bool,
  hazThreeByteVEX:                      bool,
  theREX:                               u8,
  hazTwoByteOpcode:                     bool,
  theOpcode:                            Option<u8>,
  theModRegRM:                          Option<u8>,
  theSIBByte:                           Option<u8>,
  immediateLength:                      usize,
  immediateValue:                       i128,
}

impl Instruction
{
  pub fn getBranchHint                  ( &self )     ->  u8                          { self.theBranchHint                                }
  pub fn getImmediate                   ( &self )     ->  ( usize, i128 )             { ( self.immediateLength, self.immediateValue )     }
  pub fn getLineNumber                  ( &self )     ->  usize                       { self.line                                         }
  pub fn getModRegRM                    ( &self )     ->  Option<u8>                  { self.theModRegRM                                  }
  pub fn getOpcode                      ( &self )     ->  Option<u8>                  { self.theOpcode                                    }
  pub fn getOperands                    ( &self )     ->  Vec<OperandType>            { self.operands.clone()                             }
  pub fn getOperandRefs                 ( &mut self ) ->  &mut Vec<OperandType>       { &mut self.operands                                }
  pub fn getRepeat                      ( &self )     ->  u8                          { self.theRepeat                                    }
  pub fn getREX                         ( &self )     ->  u8                          { self.theREX                                       }
  pub fn getSegmentOverride             ( &self )     ->  u8                          { self.theSegmentOverride                           }
  pub fn getSIBByte                     ( &self )     ->  Option<u8>                  { self.theSIBByte                                   }
  pub fn getType                        ( &self )     ->  InstructionType             { self.instruction.clone()                          }

  pub fn hazAddressSizeOverride         ( &self )     ->  bool                        { self.hazAddressSizeOverride                       }
  pub fn hazBranchHint                  ( &self )     ->  bool                        { self.theBranchHint          !=  0                 }
  pub fn hazLock                        ( &self )     ->  bool                        { self.hazLock                                      }
  pub fn hazOperandSizeOverride         ( &self )     ->  bool                        { self.hazOperandSizeOverride                       }
  pub fn hazRepeat                      ( &self )     ->  bool                        { self.theRepeat              !=  0                 }
  pub fn hazREX                         ( &self )     ->  bool                        { self.theREX                 !=  0                 }
  pub fn hazSegmentOverride             ( &self )     ->  bool                        { self.theSegmentOverride     !=  0                 }
  pub fn hazThreeByteVEX                ( &self )     ->  bool                        { self.hazThreeByteVEX                              }
  pub fn hazThreeByteXOP                ( &self )     ->  bool                        { self.hazThreeByteXOP                              }
  pub fn hazTwoByteOpcode               ( &self )     ->  bool                        { self.hazTwoByteOpcode                             }
  pub fn hazTwoByteVEX                  ( &self )     ->  bool                        { self.hazTwoByteVEX                                }

  pub fn orOperandSize                  ( &mut  self, size:     usize               ) { self.size                   |=  size;             }

  pub fn setAddress                     ( &mut  self, address:  InstructionAddress  ) { self.address                =   address;          }
  pub fn setAddressSizeOverride         ( &mut  self, value:    bool                ) { self.hazAddressSizeOverride =   value;            }
  pub fn setBranchHint                  ( &mut  self, value:    u8                  ) { self.theBranchHint          =   value;            }
  pub fn setImmediate
  (
    &mut  self,
    length:                             usize,
    value:                              i128
  )
  {
    self.immediateLength                =   length;
    self.immediateValue                 =   value;
  }
  pub fn setImmediateLength             ( &mut  self, value:    usize               ) { self.immediateLength        =   value;            }
  pub fn setLock                        ( &mut  self, value:    bool                ) { self.hazLock                =   value;            }
  pub fn setModRegRM                    ( &mut  self, value:    u8                  ) { self.theModRegRM            =   Some ( value  );  }
  pub fn setOpcode                      ( &mut  self, opcode:   u8                  ) { self.theOpcode              =   Some ( opcode );  }
  pub fn setOperandSizeOverride         ( &mut  self, value:    bool                ) { self.hazOperandSizeOverride =   value;            }
  pub fn setRepeat                      ( &mut  self, value:    u8                  ) { self.theRepeat              =   value;            }
  pub fn setREX                         ( &mut  self, value:    u8                  ) { self.theREX                 =   value;            }
  pub fn setSegmentOverride             ( &mut  self, value:    u8                  ) { self.theSegmentOverride     =   value;            }
  pub fn setSIBByte                     ( &mut  self, value:    u8                  ) { self.theSIBByte             =   Some ( value  );  }
  pub fn setThreeByteVEX                ( &mut  self, value:    bool                ) { self.hazThreeByteVEX        =   value;            }
  pub fn setThreeByteXOP                ( &mut  self, value:    bool                ) { self.hazThreeByteXOP        =   value;            }
  pub fn setTwoByteOpcode               ( &mut  self, value:    bool                ) { self.hazTwoByteOpcode       =   value;            }
  pub fn setTwoByteVEX                  ( &mut  self, value:    bool                ) { self.hazTwoByteVEX          =   value;            }

  pub fn fail
  (
    &self,
    message:                            String,
  ) -> Result<Option<usize>, String>
  {
    print!  ( "Line {}: ", self.line );
    //Self::printType ( &self.instruction );
    print! ( "{:?}", &self.instruction );
    for   operand                       in  self.getOperands()
    {
      operand.print ( self.size );
    }
    println!  ( "" );
    Err ( message )
  }

  pub fn print
  (
    &self,
  )
  {
    if self.instruction > InstructionType::ActualInstruction
    {
      if let InstructionAddress::Some { base, offs } = self.address
      {
        print!  ( "{:04x}:{:016x} ", base, offs );
      }
      else
      {
        print!  ( "None:None             " );
      }
    }
    else
    {
      print!    ( "                      " );
    }
    //Self::printType ( &self.instruction );
    print!      ( "{:?}", &self.instruction );
    for   operand                       in  self.getOperands()
    {
      operand.print ( self.size );
    }
    println!    ( "" );
  }

  pub fn printType
  (
    theType:                            &InstructionType,
  )
  {
    match theType
    {
      InstructionType::Label            ( identifier  )
      =>  print!  ( "label {}", identifier  ),
      InstructionType::ADD
      =>  print!  ( "add"                   ),
      InstructionType::OR
      =>  print!  ( "or "                   ),
      InstructionType::ADC
      =>  print!  ( "adc"                   ),
      InstructionType::SBB
      =>  print!  ( "sbb"                   ),
      InstructionType::AND
      =>  print!  ( "and"                   ),
      InstructionType::SUB
      =>  print!  ( "sub"                   ),
      InstructionType::XOR
      =>  print!  ( "xor"                   ),
      InstructionType::CMP
      =>  print!  ( "cmp"                   ),
      _
      =>  print!  ( "???"                   ),
    }
  }
}

pub fn Instruction
(
  line:                                 usize,
  size:                                 usize,
  instruction:                          InstructionType,
  operands:                             Vec<OperandType>,
) -> Instruction
{
  Instruction
  {
    //  for debugging
    line:                               line,
    //  for input
    size:                               size,
    instruction:                        instruction,
    operands:                           operands,
    //  for processing, initialised empty/invalid
    address:                            InstructionAddress::None,
    hazLock:                            false,
    theRepeat:                          0,
    theSegmentOverride:                 0,
    theBranchHint:                      0,
    hazOperandSizeOverride:             false,
    hazAddressSizeOverride:             false,
    hazThreeByteXOP:                    false,
    hazTwoByteVEX:                      false,
    hazThreeByteVEX:                    false,
    theREX:                             0,
    hazTwoByteOpcode:                   false,
    theOpcode:                          None,
    theModRegRM:                        None,
    theSIBByte:                         None,
    immediateLength:                    0,
    immediateValue:                     0,
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

#[derive(Clone,Debug,PartialEq,PartialOrd)]
pub enum InstructionType
{
  Label                                 ( usize ),
  ActualInstruction,
  AAA,
  AAS,
  ADC,
  ADD,
  AND,
  CBW,
  CLC,
  CLD,
  CLI,
  CMC,
  CMP,
  CMPSB,
  CWD,
  DAA,
  DAS,
  HLT,
  INT3,
  INTO,
  IRET,
  LAHF,
  LODSB,
  MOVSB,
  OR,
  POPF,
  PUSHF,
  SAHF,
  SALC,
  SBB,
  SCASB,
  STC,
  STD,
  STI,
  STOSB,
  SUB,
  WAIT,
  XLAT,
  XOR,
}

pub const AddressSizeOverride:      u8  =   0x67;
pub const BranchTaken:              u8  =   0x3e;
pub const BranchNotTaken:           u8  =   0x2e;
pub const Lock:                     u8  =   0xf0;
pub const OperandSizeOverride:      u8  =   0x66;
pub const Repeat:                   u8  =   0xf3;
pub const RepeatEqual:              u8  =   Repeat;
pub const RepeatZero:               u8  =   Repeat;
pub const RepeatNotEqual:           u8  =   0xf2;
pub const RepeatNotZero:            u8  =   RepeatNotEqual;
pub const SegmentOverrideCS:        u8  =   0x26;
pub const SegmentOverrideSS:        u8  =   0x2e;
pub const SegmentOverrideDS:        u8  =   0x36;
pub const SegmentOverrideES:        u8  =   0x3e;
pub const SegmentOverrideFS:        u8  =   0x64;
pub const SegmentOverrideGS:        u8  =   0x65;
pub const ThreeByteXOP:             u8  =   0x8f;
pub const TwoByteVEX:               u8  =   0xc5;
pub const ThreeByteVEX:             u8  =   0xc4;
pub const TwoByteOpcode:            u8  =   0x0f;