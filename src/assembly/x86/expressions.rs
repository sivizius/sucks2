use super::
{
  X86,
  operands::
  {
    Operand,
    OperandType
  },
};

#[derive(Clone,Debug)]
pub struct Expression ( pub Vec<ExpressionToken> );

impl Operand for Expression
{
  fn this   ( self ) -> ( OperandType, usize )  { ( OperandType::Expression ( self ), 0 ) }
}

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
            let mut tmp2                =   Expression::calculate ( &mut stack )?;
            let mut tmp1                =   Expression::calculate ( &mut stack )?;
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
      =>  {
            println!        ( "_{:?}_", token );
            unimplemented!  (                 );
          },
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
      =>  ( Some  ( 0 ),      OperandType::Constant               ( *value                        ) ),
      [ ExpressionToken::GeneralPurposeRegister { rex, size, number } ]
      =>  ( Some  ( *size ),  OperandType::GeneralPurposeRegister { rex:  *rex, number:  *number  } ),
      [ ExpressionToken::SegmentRegister        ( register ) ]
      =>  ( Some  ( 2 ),      OperandType::SegmentRegister        ( *register                     ) ),
      _
      =>  ( None,             OperandType::Expression             ( Expression  ( stack )         ) ),
    }
  }
  pub fn to_string
  (
    &self
  ) ->  String
  {
    let mut output                      =   "".to_string();
    for token                           in  &self.0
    {
      if output != ""
      {
        output                          +=  " "
      }
      match token
      {
        ExpressionToken::Constant               ( value               ) =>  output  +=  &format! ( "{}", value ),
        ExpressionToken::GeneralPurposeRegister { rex,  size, number  } =>  output  +=  &OperandType::GeneralPurposeRegister  { rex:  *rex, number: *number }.to_string ( *size ),
        ExpressionToken::SegmentRegister        ( register            ) =>  output  +=  &OperandType::SegmentRegister         ( *register                   ).to_string ( 2     ),
        ExpressionToken::Neg                                            =>  output  +=  "~",
        ExpressionToken::Add                                            =>  output  +=  "+",
        ExpressionToken::Substract                                      =>  output  +=  "-",
        ExpressionToken::Multiply                                       =>  output  +=  "*",
        ExpressionToken::Divide                                         =>  output  +=  "/",
        ExpressionToken::Modulo                                         =>  output  +=  "%",
        ExpressionToken::BitwiseNot                                     =>  output  +=  "!",
        ExpressionToken::BitwiseAnd                                     =>  output  +=  "&",
        ExpressionToken::BitwiseOr                                      =>  output  +=  "|",
        ExpressionToken::BitwiseXor                                     =>  output  +=  "^",
        ExpressionToken::LogicalNot                                     =>  output  +=  "!!",
        ExpressionToken::LogicalAnd                                     =>  output  +=  "&&",
        ExpressionToken::LogicalOr                                      =>  output  +=  "||",
        ExpressionToken::LogicalXor                                     =>  output  +=  "^^",
        _                                                               =>  output  +=  "…",
      };
    }
    output
  }
}

#[derive(Clone,Debug,PartialEq,PartialOrd)]
pub enum ExpressionToken
{
  //  Operands
  Constant                              ( i128  ),
  GeneralPurposeRegister
  {
    rex:                                bool,               //  true for spl, bpl, sil and dil
    size:                               usize,
    number:                             u8,
  },
  SegmentRegister                       ( u8    ),
  Label                                 ( usize ),
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
  ( al                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 1,  number: 0 } };
  ( cl                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 1,  number: 1 } };
  ( dl                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 1,  number: 2 } };
  ( bl                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 1,  number: 3 } };
  ( ah                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 1,  number: 4 } };
  ( ch                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 1,  number: 5 } };
  ( dh                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 1,  number: 6 } };
  ( bh                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 1,  number: 7 } };
  ( spl                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  true,   size: 1,  number: 4 } };
  ( bpl                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  true,   size: 1,  number: 5 } };
  ( sil                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  true,   size: 1,  number: 6 } };
  ( dil                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  true,   size: 1,  number: 7 } };
  ( ax                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 2,  number: 0 } };
  ( cx                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 2,  number: 1 } };
  ( dx                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 2,  number: 2 } };
  ( bx                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 2,  number: 3 } };
  ( sp                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 2,  number: 4 } };
  ( bp                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 2,  number: 5 } };
  ( si                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 2,  number: 6 } };
  ( di                  )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 2,  number: 7 } };
  ( eax                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 4,  number: 0 } };
  ( ecx                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 4,  number: 1 } };
  ( edx                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 4,  number: 2 } };
  ( ebx                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 4,  number: 3 } };
  ( esp                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 4,  number: 4 } };
  ( ebp                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 4,  number: 5 } };
  ( esi                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 4,  number: 6 } };
  ( edi                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 4,  number: 7 } };
  ( rax                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 0 } };
  ( rcx                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 1 } };
  ( rdx                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 2 } };
  ( rbx                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 3 } };
  ( rsp                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 4 } };
  ( rbp                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 5 } };
  ( rsi                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 6 } };
  ( rdi                 )               =>  { ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 8,  number: 7 } };
  ( cs                  )               =>  { ExpressionToken::SegmentRegister        ( 0                                 ) };
  ( ss                  )               =>  { ExpressionToken::SegmentRegister        ( 1                                 ) };
  ( ds                  )               =>  { ExpressionToken::SegmentRegister        ( 2                                 ) };
  ( es                  )               =>  { ExpressionToken::SegmentRegister        ( 3                                 ) };
  ( fs                  )               =>  { ExpressionToken::SegmentRegister        ( 4                                 ) };
  ( gs                  )               =>  { ExpressionToken::SegmentRegister        ( 5                                 ) };
  ( $value:literal      )               =>  { ExpressionToken::Constant               ( $value                            ) };
}

#[macro_export]
macro_rules! expression
{
  ( $( $token:tt )* )                   =>  { Expression ( vec![ $(  nextToken!  ( $token  ),  )*  ] ) };
}
