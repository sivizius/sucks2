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
  _nickname:                            String,
  _password:                            String
}

pub fn User
(
  nickname:                             &str,
  password:                             &str,
) -> User
{
  User
  {
    _nickname:                          nickname.to_string(),
    _password:                          password.to_string(),
  }
}

impl User
{

}
