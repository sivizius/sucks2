use super::
{
  Instruction,
  InstructionType,
  super::
  {
    X86,
  },
};

macro_rules! theInstruction
{
  (
    $theName:ident,
    $theInstruction:expr
  )
  =>  {
        pub fn $theName
        (
          mut self,
        ) -> Self
        {
          self.instructions.push
          (
            Instruction
            (
              self.line,
              self.features,
              0,
              $theInstruction,
              vec!  ( ),
            )
          );
          self.line                     +=  1;
          self
        }
      }
}

impl X86
{
  theInstruction! ( aaa,    InstructionType::AAA    );
  theInstruction! ( aas,    InstructionType::AAS    );
  theInstruction! ( cbw,    InstructionType::CBW    );
  theInstruction! ( clc,    InstructionType::CLC    );
  theInstruction! ( cld,    InstructionType::CLD    );
  theInstruction! ( cli,    InstructionType::CLI    );
  theInstruction! ( cmc,    InstructionType::CMC    );
  theInstruction! ( cmpsb,  InstructionType::CMPSB  );
  theInstruction! ( cmpsw,  InstructionType::CMPSW  );
  theInstruction! ( cwd,    InstructionType::CWD    );
  theInstruction! ( daa,    InstructionType::DAA    );
  theInstruction! ( das,    InstructionType::DAS    );
  theInstruction! ( hlt,    InstructionType::HLT    );
  theInstruction! ( int3,   InstructionType::INT3   );
  theInstruction! ( into,   InstructionType::INTO   );
  theInstruction! ( iret,   InstructionType::IRET   );
  theInstruction! ( lahf,   InstructionType::LAHF   );
  theInstruction! ( lodsb,  InstructionType::LODSB  );
  theInstruction! ( lodsw,  InstructionType::LODSW  );
  theInstruction! ( movsb,  InstructionType::MOVSB  );
  theInstruction! ( movsw,  InstructionType::MOVSW  );
  theInstruction! ( popf,   InstructionType::POPF   );
  theInstruction! ( pushf,  InstructionType::PUSHF  );
  theInstruction! ( sahf,   InstructionType::SAHF   );
  theInstruction! ( salc,   InstructionType::SALC   );
  theInstruction! ( scasb,  InstructionType::SCASB  );
  theInstruction! ( scasw,  InstructionType::SCASW  );
  theInstruction! ( stc,    InstructionType::STC    );
  theInstruction! ( std,    InstructionType::STD    );
  theInstruction! ( sti,    InstructionType::STI    );
  theInstruction! ( stosb,  InstructionType::STOSB  );
  theInstruction! ( stosw,  InstructionType::STOSW  );
  theInstruction! ( wait,   InstructionType::WAIT   );
  theInstruction! ( xlat,   InstructionType::XLAT   );
}

impl  Instruction
{
  pub fn compileZeroOperandInstruction
  (
    &mut self,
    opcode:                             u8,
  ) -> Result<Option<usize>, String>
  {
    self.setOpcode  ( opcode  );
    Ok  ( Some  ( 1 ) )
  }
}