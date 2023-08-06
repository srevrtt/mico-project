
/*
  This file contains the logic for creating new tokens for the MICO IR lexer
*/

/// 
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
  pub name: String,
  pub val: String
}

impl Token {
  pub fn new(name: &str, val: &str) -> Self {
    Self {
      name: name.to_string(),
      val: val.to_string()
    }
  }
}
