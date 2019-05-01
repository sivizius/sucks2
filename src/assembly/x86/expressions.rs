use super::
{
  X86,
  operands::
  {
    Operand,
    OperandType
  },
};

#[derive(Clone,Debug,PartialEq,PartialOrd)]
pub enum ExpressionToken
{
  //  Operands
  Constant                              ( i128  ),
  Label                                 ( usize ),
  GeneralPurposeRegister
  {
    rex:                                bool,               //  true for spl, bpl, sil and dil
    size:                               usize,
    number:                             u8,
  },
  //  One Operand Operators
  Neg,
  BitwiseNot,
  LogicalNot,
  //  Two Operand Operators
  Add,
  Substract,
  Multiply,
  Divide,
  Modulo,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  LogicalAnd,
  LogicalOr,
  LogicalXor,
}

impl Operand for Expression
{
  fn this   ( self ) -> ( OperandType, usize )  { ( OperandType::Expression ( self ), 0 ) }
}

#[derive(Clone,Debug)]
pub struct Expression ( pub Vec<ExpressionToken> );

impl Expression
{
  fn calculate
  (
    mut stack:                          &mut Vec<ExpressionToken>,
  ) -> Option<Vec<ExpressionToken>>
  {
    let token                           =   stack.pop()?;
    match token
    {
      ExpressionToken::Constant ( value )
      =>  {
            Some  ( vec!  ( ExpressionToken::Constant ( value ) ) )
          },
      _
      if  token >= ExpressionToken::Add
      =>  {
            let mut tmp2              =   Expression::calculate ( &mut stack )?;
            let mut tmp1              =   Expression::calculate ( &mut stack )?;
            if let  (
                      [ ExpressionToken::Constant ( val1  ) ],
                      [ ExpressionToken::Constant ( val2  ) ],
                    )
                    = ( tmp1.as_slice(), tmp2.as_slice() )
            {
              Some
              (
                vec!
                (
                  ExpressionToken::Constant
                  (
                    match token
                    {
                      ExpressionToken::Add                  =>  val1 + val2,
                      ExpressionToken::Substract            =>  val1 - val2,
                      ExpressionToken::Multiply             =>  val1 * val2,
                      ExpressionToken::Divide               =>  val1 / val2,
                      ExpressionToken::Modulo               =>  val1 % val2,
                      ExpressionToken::BitwiseAnd           =>  val1 & val2,
                      ExpressionToken::BitwiseOr            =>  val1 | val2,
                      ExpressionToken::BitwiseXor           =>  val1 ^ val2,
                      ExpressionToken::LogicalAnd           =>  (( *val1 != 0 ) && ( *val2 != 0 )) as i128,
                      ExpressionToken::LogicalOr            =>  (( *val1 != 0 ) || ( *val2 != 0 )) as i128,
                      ExpressionToken::LogicalXor           =>  (( *val1 != 0 ) ^ ( *val2 != 0 )) as i128,
                      _                                     =>  unreachable!(),
                    }
                  )
                )
              )
            }
            else
            {
              tmp1.append ( &mut tmp2 );
              tmp1.push   ( token     );
              Some        ( tmp1      )
            }
          },
      _
      if  token >= ExpressionToken::Neg
      =>  {
            let mut tmp1              =   Expression::calculate ( &mut stack )?;
            if let  [ ExpressionToken::Constant ( val1  ) ] = tmp1.as_slice()
            {
              Some
              (
                vec!
                (
                  ExpressionToken::Constant
                  (
                    match token
                    {
                      ExpressionToken::Neg                  =>  -val1,
                      ExpressionToken::BitwiseNot           =>  !val1,
                      ExpressionToken::LogicalNot           =>  ( *val1 == 0 ) as i128,
                      _                                     =>  unreachable!(),
                    }
                  )
                )
              )
            }
            else
            {
              tmp1.push   ( token       );
              Some        ( tmp1        )
            }
          },
      _
      =>  unimplemented!(),
    }
  }
  pub fn solve
  (
    &self
  ) -> ( Option<usize>, OperandType )
  {
    let stack                           =   Expression::calculate ( &mut self.0.clone() ).unwrap();
    match stack.as_slice()
    {
      [ ExpressionToken::Constant ( value ) ]
      =>  ( Some  ( 0 ),  OperandType::Constant   ( *value                ) ),
      _
      =>  ( None,         OperandType::Expression ( Expression  ( stack ) ) ),
    }
  }
}

#[macro_export]
macro_rules! nextToken
{
  ( ~                   )               =>  { ExpressionToken::Neg                                                          };
  ( +                   )               =>  { ExpressionToken::Add                                                          };
  ( -                   )               =>  { ExpressionToken::Substract                                                    };
  ( *                   )               =>  { ExpressionToken::Multiply                                                     };
  ( /                   )               =>  { ExpressionToken::Divide                                                       };
  ( !                   )               =>  { ExpressionToken::BitwiseNot                                                   };
  ( &                   )               =>  { ExpressionToken::BitwiseAnd                                                   };
  ( |                   )               =>  { ExpressionToken::BitwiseOr                                                    };
  ( ^                   )               =>  { ExpressionToken::BitwiseXor                                                   };
  ( !!                  )               =>  { ExpressionToken::LogicalNot                                                   };
  ( &&                  )               =>  { ExpressionToken::LogicalAnd                                                   };
  ( ||                  )               =>  { ExpressionToken::LogicalOr                                                    };
  ( ^^                  )               =>  { ExpressionToken::LogicalXor                                                   };
  ( not                 )               =>  { ExpressionToken::LogicalNot                                                   };
  ( and                 )               =>  { ExpressionToken::LogicalAnd                                                   };
  ( or                  )               =>  { ExpressionToken::LogicalOr                                                    };
  ( xor                 )               =>  { ExpressionToken::LogicalXor                                                   };
  ( rax                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 0 } };
  ( rcx                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 1 } };
  ( rdx                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 2 } };
  ( rbx                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 3 } };
  ( rsp                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 4 } };
  ( rbp                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 5 } };
  ( rsi                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 6 } };
  ( rdi                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 7 } };
  ( $value:literal      )               =>  { ExpressionToken::Constant               ( $value                            ) };
}

#[macro_export]
macro_rules! expression
{
  ( $( $token:tt )* )                   =>  { Expression ( vec![ $(  nextToken!  ( $token  ),  )*  ] ) };
}
