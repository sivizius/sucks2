pub mod x86;

#[allow(non_camel_case_types)]
#[derive(Copy,Clone,PartialEq,PartialOrd)]
pub enum InstructionSet
{
  i8086                                 =   0x8600,
  i186,
  i286,
  i386,
  i486,
  Pentium,
  Pentium2,
  amd64,
}

pub fn InstructionSet
(
  instructionSet:                       InstructionSet,
) -> &'static str
{
  #[allow(unreachable_patterns)]
  match instructionSet
  {
    InstructionSet::i8086               =>  { "8086 (x86)"              },
    InstructionSet::i186                =>  { "i186 (x86)"              },
    InstructionSet::i286                =>  { "i286 (x86)"              },
    InstructionSet::i386                =>  { "i386 (x86)"              },
    InstructionSet::i486                =>  { "i486 (x86)"              },
    InstructionSet::Pentium             =>  { "Pentium (x86)"           },
    InstructionSet::Pentium2            =>  { "Pentium II (x86)"        },
    _                                   =>  { "Unknown Instruction Set" },
  }
}

bitflags!
{
  pub struct AssemblyFeatures:          usize
  {
    const None                          =   0b0000_0000_0000_0000_0000_0000_0000_0000;  //  No Features

    const RandomExecutionOrder          =   0b0000_0000_0000_0000_0000_0000_0000_0001;  //  Randomise Order of Execution by moving around instructions
    const RandomFunctionOrder           =   0b0000_0000_0000_0000_0000_0000_0000_0010;  //  Randomise Order of Functions
    const RandomHeapDataOrder           =   0b0000_0000_0000_0000_0000_0000_0000_0100;  //  Randomise Order of Heap Data
    const RandomJunkInstructions        =   0b0000_0000_0000_0000_0000_0000_0000_1000;  //  Add Random Junk Instructions

    const RandomOpcode                  =   0b0000_0000_0000_0000_0000_0000_0001_0000;  //  Randomise Opcodes without changing size
    const RandomOpcodeSize              =   0b0000_0000_0000_0000_0000_0000_0010_0000;  //  Randomise Opcodes by using longer instructions
    const RandomPaddingBytes            =   0b0000_0000_0000_0000_0000_0000_0100_0000;  //  Random Bytes instead of zeros for Padding
    const RandomPaddingLength           =   0b0000_0000_0000_0000_0000_0000_1000_0000;  //  Add Random Paddings of Data

    const RandomPrefixes                =   0b0000_0000_0000_0000_0000_0001_0000_0000;  //  Add Random Prefixes
    const RandomPrefixOrder             =   0b0000_0000_0000_0000_0000_0010_0000_0000;  //  Randomise Order of Prefixes, where possible
    const RandomRegisters               =   0b0000_0000_0000_0000_0000_0100_0000_0000;  //  Randomise used Registers
    const RandomStructureOrder          =   0b0000_0000_0000_0000_0000_1000_0000_0000;  //  Randomise Order of Structure elements

    const Default                       =   0b0000_0000_0000_0000_0000_0000_0011_0000;  //  Default
  }
}

impl AssemblyFeatures
{
  pub fn hazFeature
  (
    self,
    flags:                              AssemblyFeatures,
  ) -> bool
  {
    ( self & flags ) != AssemblyFeatures::None
  }
}
