#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct UserID                       ( usize );

impl From<usize> for UserID
{
  fn from
  (
    this:                               usize
  ) -> Self
  {
    UserID ( this )
  }
}

pub struct User
{
  nickname:                             String,
  password:                             String
}

pub fn User
(
  nickname:                             &str,
  password:                             &str,
) -> User
{
  User
  {
    nickname:                           nickname.to_string(),
    password:                           password.to_string(),
  }
}

impl User
{
  
}
