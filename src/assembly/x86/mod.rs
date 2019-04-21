#[macro_use]
mod instructions;
mod operands;
#[macro_use]
mod registers;

pub use self::
{
  instructions::
  {
    Instruction,
    InstructionType,
  },
};

pub use super::
{
  InstructionSet,
};

use std::
{
  collections::
  {
    HashMap,
  },
  string::
  {
    String,
  },
};

pub struct X86
{
  instructions:                         Vec<Instruction>,
  identifiers:                          HashMap<String, usize>,
  line:                                 usize,
}

pub fn X86
(
) -> X86
{
  X86
  {
    instructions:                       vec!(),
    identifiers:                        HashMap::new(),
    line:                               0,
  }
}

impl X86
{
  pub fn label
  (
    mut self,
    name:                               &str,
  ) -> Self
  {
    self.instructions.push
    (
      Instruction
      (
        self.line,
        "label",
        0,
        Some ( 0 ),
        InstructionType::Label ( self.identifiers.len() ),
        vec!(),
      )
    );
    self.line                           += 1;
    self.identifiers.insert
    (
      String::from( name ),
      self.identifiers.len()
    );
    self
  }

  #[allow(unused_mut)]
  pub fn compile
  (
    self,
    mut architecture:                   InstructionSet,
    mut operandSize:                    usize,
    mut addressSize:                    usize,
  ) -> Result<usize, String>
  {
    if  ( architecture < InstructionSet::i386 )
    &&  (
          ( operandSize != 16 ) || ( addressSize != 16 )
        )
    {
      return Err ( format!( "Instruction Set ›{}‹ is 16 Bit Only", InstructionSet( architecture ) ) );
    }

    let mut labels: Vec<Option<usize>>  =   vec!();
    labels.resize
    (
      self.identifiers.len(),
      None
    );

    let mut address:          usize     =   0;
    for mut instruction                 in  self.instructions
    {
      match instruction.getOpcode()
      {
        InstructionType::Label          ( identifier )
        =>  {
              if identifier < self.identifiers.len()
              {
                labels[ identifier ]    =   Some ( address );
              }
              else
              {
                return  Err
                        (
                          format!
                          (
                            "Invalid Label Number ›{}‹ on Line {}",
                            identifier,
                            instruction.getLineNumber()
                          )
                        );
              }
            },
        InstructionType::SimpleMath     ( opcode )
        =>  {
              X86::compileSimpleMathInstruction
              (
                &mut instruction,
                &mut architecture,
                &mut operandSize,
                &mut addressSize,
                opcode
              )?;
            },
        _
        =>  {
            },
      }
    }

    Ok(2)
  }
}
