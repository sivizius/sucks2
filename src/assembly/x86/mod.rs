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
    InstructionAddress,
    InstructionType,
  },
  operands::
  {
    OperandType,
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
    mut self,
    mut architecture:                   InstructionSet,
    mut operandSize:                    usize,
    mut addressSize:                    usize,
  ) -> Result<Box<[u8]>, String>
  {
    let mut output:             Vec<u8> =   vec!();

    if  ( architecture < InstructionSet::i386 )
    &&  (
          ( operandSize != 16 ) || ( addressSize != 16 )
        )
    {
      return Err ( format!( "Instruction Set ›{}‹ is 16 Bit Only", InstructionSet( architecture ) ) );
    }

    let mut labels:                     Vec<InstructionAddress> 
                                        =   vec!();
    labels.resize
    (
      self.identifiers.len(),
      InstructionAddress::None
    );

    println!  ( "before" );
    for mut instruction                 in  &mut self.instructions
    {
      instruction.print       ();
    }

    let mut address
    = InstructionAddress::Some
      {
        base:                           0,
        offs:                           0,
      };

    println!  ( "after" );
    for mut instruction                 in  &mut self.instructions
    {
      let mut length                    =   Some ( 0 );
      match instruction.getOpcode()
      {
        InstructionType::Label          ( identifier )
        =>  {
              if identifier < self.identifiers.len()
              {
                labels[ identifier ]    =   address;
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
              length
              = X86::compileSimpleMathInstruction
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
              return  Err
                      (
                        "Unexpected Instruction. This should not happen here!".to_string()
                      );
            },
      }

      instruction.setAddress  ( address );
      address.add             ( length  )?;
      instruction.setLength   ( length  );
      instruction.print       ();
    }

    for instruction                     in  self.instructions
    {
      match instruction.getOpcode()
      {
        InstructionType::Label          ( identifier )
        =>  {
              //just ignore
            },
        InstructionType::OneByte        ( opcode )
        =>  {
              output.push ( opcode );
            },
        _
        =>  {
              return  Err
                      (
                        "Unexpected Instruction. This should never happen here!".to_string()
                      );
            },
      }
      for operand                       in  instruction.getOperands()
      {
        match operand
        {
          OperandType::Byte             ( immediate )
          =>  {
                output.push ( ( ( immediate >>  0 ) & 0xff ) as u8 );
              },
          OperandType::Word             ( immediate )
          =>  {
                output.push ( ( ( immediate >>  0 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >>  8 ) & 0xff ) as u8 );
              },
          OperandType::DWord            ( immediate )
          =>  {
                output.push ( ( ( immediate >>  0 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >>  8 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 16 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 24 ) & 0xff ) as u8 );
              },
          OperandType::QWord            ( immediate )
          =>  {
                output.push ( ( ( immediate >>  0 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >>  8 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 16 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 24 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 32 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 40 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 48 ) & 0xff ) as u8 );
                output.push ( ( ( immediate >> 56 ) & 0xff ) as u8 );
              },
          _
          =>  {
                return  Err
                        (
                          format!
                          (
                            "Unexpected Operand »{}«. This should never happen here!",
                            operand.to_string(),
                          )
                        );
              },
        }
      }
    }

    Ok ( output.into_boxed_slice() )
  }

  pub fn list
  (
    &self,
  )
  {
    for     instruction                 in  &self.instructions
    {
      instruction.print ();
    }
  }
}
