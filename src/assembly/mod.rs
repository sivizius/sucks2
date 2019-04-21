pub mod x86;

#[allow(non_camel_case_types)]
#[derive(PartialEq,PartialOrd)]
pub enum InstructionSet
{
  i8086                                 =   0x8600,
  i186,
  i286,
  i386,
  i486,
  Pentium,
  Pentium2,
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