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
  //  for debugging, writable once
  line:                                 usize,
  //  for input, writeable once
  size:                                 usize,
  instruction:                          InstructionType,
  operands:                             Vec<OperandType>,
  //  for processing, initialised empty/invalid
  address:                              InstructionAddress,
  length:                               Option<usize>,
  parts:                                Vec<InstructionPart>,
}

impl Instruction
{
  pub fn addPart
  (
    &mut  self,
    part:                               InstructionPart,
  )
  {
    self.parts.push ( part );
  }

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
      operand.print ( self.size, false );
    }
    println!  ( "" );
    Err ( message )
  }

  pub fn getLineNumber
  (
    &self,
  ) -> usize
  {
    self.line
  }

  pub fn getOperands
  (
    &self,
  ) -> Vec<OperandType>
  {
    self.operands.clone()
  }

  pub fn getParts
  (
    &self,
  ) -> Vec<InstructionPart>
  {
    self.parts.clone()
  }

  pub fn getType
  (
    &self,
  ) -> InstructionType
  {
    self.instruction.clone()
  }

  pub fn print
  (
    &self,
  )
  {
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
    //Self::printType ( &self.instruction );
    print! ( "{:?}", &self.instruction );
    for   operand                       in  self.getOperands()
    {
      operand.print ( self.size, false );
    }
    println!  ( "" );
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
    length:                             None,
    parts:                              vec!(),
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

#[derive(Clone,Debug)]
pub enum InstructionType
{
  Label                                 ( usize ),
  ADD,
  OR,
  ADC,
  SBB,
  AND,
  SUB,
  XOR,
  CMP,
}

#[derive(Clone)]
pub enum InstructionPart
{
  Lock,
  Repeat,
  RepeatNot,
  SegmentOverride                       ( u8 ),
  BranchTaken,
  BranchNotTaken,
  OperandSizeOverride,
  AddressSizeOverride,
  ThreeByteXOP,
  TwoByteVEX,
  ThreeByteVEX,
  REX                                   ( u8 ),
  OneByteInstruction                    ( u8 ),
  TwoByteInstruction                    ( u8 ),
  ModRegRM                              ( u8 ),
  SIBByte                               ( u8 ),
  ImmediateByte                         ( i128 ),
  ImmediateWord                         ( i128 ),
  ImmediateDWord                        ( i128 ),
  ImmediateQWord                        ( i128 ),
}
