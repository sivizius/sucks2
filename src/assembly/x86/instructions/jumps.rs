use super::
{
  Instruction,
  InstructionType,
  super::
  {
    InstructionSet,
    X86,
    operands::
    {
      Operand,
      OperandType,
    },
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
          label:                        impl Operand,
        ) -> Self
        {
          let ( thisLabel,  _ )         =   label.this();
          self.instructions.push
          (
            Instruction
            (
              self.line,
              self.features,
              0,
              $theInstruction,
              vec!  ( thisLabel ),
            )
          );
          self.line                     +=  1;
          self
        }
      }
}

impl  X86
{
  theInstruction! ( jb,     InstructionType::JB     );
  theInstruction! ( jbe,    InstructionType::JBE    );
  theInstruction! ( jcxz,   InstructionType::JCXZ   );
  theInstruction! ( je,     InstructionType::JE     );
  theInstruction! ( jl,     InstructionType::JL     );
  theInstruction! ( jle,    InstructionType::JLE    );
  theInstruction! ( jnb,    InstructionType::JNB    );
  theInstruction! ( jnbe,   InstructionType::JNBE   );
  theInstruction! ( jne,    InstructionType::JNE    );
  theInstruction! ( jnl,    InstructionType::JNL    );
  theInstruction! ( jnle,   InstructionType::JNLE   );
  theInstruction! ( jno,    InstructionType::JNO    );
  theInstruction! ( jnp,    InstructionType::JNP    );
  theInstruction! ( jns,    InstructionType::JNS    );
  theInstruction! ( jo,     InstructionType::JO     );
  theInstruction! ( jp,     InstructionType::JP     );
  theInstruction! ( js,     InstructionType::JS     );
  theInstruction! ( jz,     InstructionType::JE     );
  theInstruction! ( looop,  InstructionType::LOOP   );
  theInstruction! ( loopz,  InstructionType::LOOPZ  );
  theInstruction! ( loopnz, InstructionType::LOOPNZ );
}

impl  Instruction
{
  pub fn compileJumpInstruction
  (
    &mut self,
    architecture:                       InstructionSet,
    operandSize:                        usize,
    opcode:                             u8,
  ) -> Result<Option<usize>, String>
  {
    if self.operands.len() == 1
    {
      match &self.operands [ 0 ]
      {
        OperandType::Displacement ( mut displacement  )
        =>  {
              displacement              -=  2;
              if  displacement  >= -0x80
              &&  displacement  <=  0x7f
              {
                self.setOpcode    ( opcode,               );
                self.setImmediate ( 1,      displacement, );
                Ok  ( Some  ( 2 ) )
              }
              else
              {
                Err
                (
                  format!
                  (
                    "Destination of Jump to far away: {}",
                    displacement,
                  )
                )
              }
            },
        OperandType::Reference    ( _                 )
        =>  {
              Ok  ( Some  ( 2 ) )
            },
        _
        =>  {
              self.fail
              (
                format!
                (
                  "Invalid Combination of Arguments ›{}‹, ›{}‹",
                  self.operands [ 0 ].to_string  ( self.size  ),
                  self.operands [ 1 ].to_string  ( self.size  ),
                )
              )
            },
      }
    }
    else
    {
      self.fail
      (
        format!
        (
          "Instruction Must Take Exactly One Argument, got {}",
          self.operands.len(),
        )
      )
    }
  }
}