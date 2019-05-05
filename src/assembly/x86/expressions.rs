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
  fn dimension
  (
    mut stack:                          &mut Vec<ExpressionToken>,
    item:                               &ExpressionToken,
  ) -> Result<( i128, Vec<ExpressionToken>  ), &'static str>
  {
    if let  Some  ( token ) = stack.pop()
    {
      if token == *item
      {
        Ok                              ( ( 1,            vec!  (       ) ) )   //←
      }
      else
      {
        match token
        {
          ExpressionToken::Constant               ( _   ) |
          ExpressionToken::GeneralPurposeRegister { ..  } |
          ExpressionToken::SegmentRegister        ( _   )
          =>  Ok                        ( ( 0,            vec!  ( token ) ) ),  //←
          ExpressionToken::Memory16               { size, segment }
          =>  unimplemented!(),
          ExpressionToken::Add        |
          ExpressionToken::Substract
          =>  {
                let ( mul2, mut rest2 ) =   Expression::dimension ( &mut stack, item  )?;
                let ( mul1, mut rest1 ) =   Expression::dimension ( &mut stack, item  )?;
                rest1.append    ( &mut rest2                  );
                if token == ExpressionToken::Add
                {
                  rest1.push    ( ExpressionToken::Add        );
                  Ok                    ( ( mul1 + mul2,  rest1           ) )   //←
                }
                else
                {
                  rest1.push    ( ExpressionToken::Substract  );
                  Ok                    ( ( mul1 - mul2,  rest1           ) )   //←
                }
              },
          ExpressionToken::Multiply
          =>  {
                let ( mul2, mut rest2 ) =   Expression::dimension ( &mut stack, item  )?;
                let ( mul1, mut rest1 ) =   Expression::dimension ( &mut stack, item  )?;
                let     val1            =   if let  [ ExpressionToken::Constant ( val1  ) ] = rest1.as_slice() { Some ( val1  ) } else  { None  };
                let     val2            =   if let  [ ExpressionToken::Constant ( val2  ) ] = rest2.as_slice() { Some ( val2  ) } else  { None  };
                if        mul2  !=  0
                &&        val1  !=  None
                {
                  //  <a> * <c·x> = <ac·x>
                  let     val1          =   val1.unwrap();
                  Ok                    ( ( val1 * mul2,  rest2           ) )   //←
                }
                else  if  mul1  !=  0
                      &&  val2  !=  None
                {
                  //  <c·x> * <b> = <bc·x>
                  let     val2          =   val2.unwrap();
                  Ok                    ( ( val2 * mul1,  rest1           ) )   //←
                }
                else
                {
                  rest1.append  ( &mut rest2                  );
                  rest1.push    ( token                       );
                  Ok                    ( ( 0,            rest1           ) )   //←
                }
              },
          ExpressionToken::Neg
          =>  {
                let ( mul1, mut rest1 ) =   Expression::dimension ( &mut stack, item  )?;
                rest1.push      ( ExpressionToken::Neg        );
                Ok                      ( ( -mul1,        rest1           ) )   //←
              },
          _
          if  token > ExpressionToken::Divide
          =>  {
                let mut tmp2            =   Expression::calculate ( &mut stack )?;
                let mut tmp1            =   Expression::calculate ( &mut stack )?;
                tmp1.append     ( &mut tmp2                   );
                tmp1.push       ( token                       );
                Ok                      ( ( 0,            tmp1            ) )   //←
              },
          _
          if  token > ExpressionToken::Neg
          =>  {
                let mut tmp1            =   Expression::calculate ( &mut stack )?;
                tmp1.push       ( token                       );
                Ok                      ( ( 0,            tmp1            ) )   //←
              }
          _
          =>  {
                println!        ( "_{:?}_", token );
                unimplemented!  (                 );
              },
        }
      }
    }
    else
    {
      Err
      (
        "operands expected, but stack is emtpy"
      )
    }
  }
  fn calculate
  (
    mut stack:                          &mut Vec<ExpressionToken>,
  ) -> Result<Vec<ExpressionToken>, &'static str>
  {
    if let  Some  ( token ) = stack.pop()
    {
      match token
      {
        ExpressionToken::Constant               ( _   ) |
        ExpressionToken::GeneralPurposeRegister { ..  } |
        ExpressionToken::SegmentRegister        ( _   )
        =>  Ok    ( vec!  ( token ) ),
        ExpressionToken::Memory16               { size, segment }
        =>  {
              let mut substack          =   Expression::calculate ( &mut stack )?;
              let ( scale,  mut rest  ) =   Expression::dimension ( &mut substack, &ExpressionToken::GeneralPurposeRegister { rex:  false,  size: 2,  number: 3 } )?; //  bx
              println!  ( "scale: {}", scale );
              Ok   ( vec! ( ExpressionToken::Constant ( 0 )) )
            },
        _
        if  token >= ExpressionToken::Add
        =>  {
              let mut tmp2              =   Expression::calculate ( &mut stack )?;
              let mut tmp1              =   Expression::calculate ( &mut stack )?;
              let     val1              =   if let  [ ExpressionToken::Constant ( val1  ) ] = tmp1.as_slice() { Some ( val1  ) } else  { None  };
              let     val2              =   if let  [ ExpressionToken::Constant ( val2  ) ] = tmp2.as_slice() { Some ( val2  ) } else  { None  };
              if  val1  !=  None
              &&  val2  !=  None
              {
                let     val1            =   val1.unwrap();
                let     val2            =   val2.unwrap();
                Ok
                (
                  vec!
                  (
                    ExpressionToken::Constant
                    (
                      match token
                      {
                        ExpressionToken::Add                =>  val1 + val2,
                        ExpressionToken::Substract          =>  val1 - val2,
                        ExpressionToken::Multiply           =>  val1 * val2,
                        ExpressionToken::Divide             =>  val1 / val2,
                        ExpressionToken::Modulo             =>  val1 % val2,
                        ExpressionToken::BitwiseAnd         =>  val1 & val2,
                        ExpressionToken::BitwiseOr          =>  val1 | val2,
                        ExpressionToken::BitwiseXor         =>  val1 ^ val2,
                        ExpressionToken::LogicalAnd         =>  ( ( *val1 !=  0 ) &&  ( *val2 !=  0 ) ) as  i128,
                        ExpressionToken::LogicalOr          =>  ( ( *val1 !=  0 ) ||  ( *val2 !=  0 ) ) as  i128,
                        ExpressionToken::LogicalXor         =>  ( ( *val1 !=  0 ) ^   ( *val2 !=  0 ) ) as  i128,
                        _                                   =>  unreachable!(),
                      }
                    )
                  )
                )
              }
              else  if  token ==  ExpressionToken::Add
                    &&  val1  ==  Some  ( &0  )
              {
                //  0 + b = b
                Ok    ( tmp2  )
              }
              else  if  token ==  ExpressionToken::Add
                    &&  val2  ==  Some  ( &0  )
              {
                //  a + 0 = a
                Ok    ( tmp1  )
              }
              else  if  token ==  ExpressionToken::Substract
                    &&  val2  ==  Some  ( &0  )
              {
                //  a - 0 = a
                Ok    ( tmp1  )
              }
              else  if  token ==  ExpressionToken::Multiply
                    &&  val1  ==  Some  ( &1  )
              {
                //  1 * b = b
                Ok    ( tmp2  )
              }
              else  if  token ==  ExpressionToken::Multiply
                    &&  val2  ==  Some  ( &1  )
              {
                //  a * 1 = a
                Ok    ( tmp1  )
              }
              else  if  token ==  ExpressionToken::Multiply
                    &&  ( val1  ==  Some  ( &0  ) ||  val2  ==  Some  ( &0  ) )
              {
                //  a * 0 = 0 * b = 0
                Ok    ( vec!  ( ExpressionToken::Constant ( 0 ) ) )
              }
              else  if  token ==  ExpressionToken::Divide
                    &&  val2  ==  Some  ( &1  )
              {
                //  a / 1 = a
                Ok    ( tmp1  )
              }
              else
              {
                tmp1.append ( &mut tmp2 );
                tmp1.push   ( token     );
                Ok          ( tmp1      )
              }
            },
        _
        if  token >= ExpressionToken::Neg
        =>  {
              let mut tmp1                =   Expression::calculate ( &mut stack )?;
              if let  [ ExpressionToken::Constant ( val1  ) ] = tmp1.as_slice()
              {
                Ok
                (
                  vec!
                  (
                    ExpressionToken::Constant
                    (
                      match token
                      {
                        ExpressionToken::Neg                =>  -val1,
                        ExpressionToken::BitwiseNot         =>  !val1,
                        ExpressionToken::LogicalNot         =>  ( *val1 == 0 ) as i128,
                        _                                   =>  unreachable!(),
                      }
                    )
                  )
                )
              }
              else
              {
                tmp1.push   ( token       );
                Ok          ( tmp1        )
              }
            },
        _
        =>  {
              println!        ( "_{:?}_", token );
              unimplemented!  (                 );
            },
      }
    }
    else
    {
      Err
      (
        "operands expected, but stack is emtpy"
      )
    }
  }
  pub fn solve
  (
    &self
  ) -> Result<( Option<usize>, OperandType ), &'static str>
  {
    let     stack                       =   Expression::calculate ( &mut self.0.clone() )?;
    match stack.as_slice()
    {
      [ ExpressionToken::Constant ( value ) ]
      =>  Ok  ( ( Some  ( 0 ),      OperandType::Constant               ( *value                        ) ) ),
      [ ExpressionToken::GeneralPurposeRegister { rex, size, number } ]
      =>  Ok  ( ( Some  ( *size ),  OperandType::GeneralPurposeRegister { rex:  *rex, number:  *number  } ) ),
      [ ExpressionToken::SegmentRegister        ( register ) ]
      =>  Ok  ( ( Some  ( 2 ),      OperandType::SegmentRegister        ( *register                     ) ) ),
      _
      =>  Ok  ( ( None,             OperandType::Expression             ( Expression  ( stack )         ) ) ),
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
        ExpressionToken::Memory16               {       size, segment } =>  output  +=  &format!
                                                                                        (
                                                                                          "mem16({}, {})",
                                                                                          match size
                                                                                          {
                                                                                            1 =>  "byte".to_string(),
                                                                                            2 =>  "word".to_string(),
                                                                                            4 =>  "dword".to_string(),
                                                                                            8 =>  "qword".to_string(),
                                                                                            _ =>  format! ( "{}", size ),
                                                                                          },
                                                                                          match segment
                                                                                          {
                                                                                            0 =>  "cs",
                                                                                            1 =>  "ss",
                                                                                            2 =>  "ds",
                                                                                            3 =>  "es",
                                                                                            4 =>  "fs",
                                                                                            5 =>  "gs",
                                                                                            _ =>  "??",
                                                                                          },
                                                                                        ),
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
        //_                                                               =>  output  +=  "…",
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
  Memory16
  {
    size:                               usize,
    segment:                            u8,
  },
  //Label                                 ( usize ),
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
