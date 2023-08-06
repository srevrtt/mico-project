/*
  This file contains the logic for tokenizing the MICO IR to later be parsed and converted to machine code
*/

use super::token::Token;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lexer {
  tkns: Vec<Token>,
  ir: String,
  idx: i32
}

impl Lexer {
  /// Skips over whitespace in the IR
  fn skip_whitespace(&mut self) {
    while self.ir.chars().nth(self.idx as usize).unwrap().is_ascii_whitespace() {
      self.idx += 1;
    }
  }

  /// Tokenizes letters in the IR in returns them as a `String`
  fn tokenize_letters(&mut self) -> String {
    let mut letters = String::new();

    while self.ir.chars().nth(self.idx as usize).unwrap().is_ascii_alphabetic() || 
    self.ir.chars().nth(self.idx as usize).unwrap() == '_' {
      letters.push(self.ir.chars().nth(self.idx as usize).unwrap());
      self.idx += 1;
    }

    letters
  }

  /// Tokenizes an integer in the IR in returns them as a `String`
  fn tokenize_integer(&mut self) -> String {
    let mut integer = String::new();

    while self.ir.chars().nth(self.idx as usize).unwrap().is_ascii_digit() {
      integer.push(self.ir.chars().nth(self.idx as usize).unwrap());
      self.idx += 1;
    }

    integer
  }

  /// Tokenizes a special symbol and appends it to the token vector
  fn tokenize_symbol(&mut self) {
    if self.ir.chars().nth(self.idx as usize).unwrap() == '$' {
      self.tkns.push(Token::new("$T_DOLLAR", "$"));
    }

    if self.ir.chars().nth(self.idx as usize).unwrap() == ';' {
      self.tkns.push(Token::new("$T_SEMICOLON", ";"));
    }

    if self.ir.chars().nth(self.idx as usize).unwrap() == '%' {
      self.tkns.push(Token::new("$T_PERCENT", "%"));
    }

    if self.ir.chars().nth(self.idx as usize).unwrap() == '{' {
      self.tkns.push(Token::new("$T_BEGIN_BRACE", "{"));
    }

    if self.ir.chars().nth(self.idx as usize).unwrap() == '}' {
      self.tkns.push(Token::new("$T_END_BRACE", "}"));
    }

    if self.ir.chars().nth(self.idx as usize).unwrap() == '>' {
      self.tkns.push(Token::new("$T_GREATER_THAN", ">"));
    }
  }

  /// Generates a stream of tokens from the IR
  pub fn new(ir: String) -> Self {
    let mut lexer = Self { tkns: vec![], ir: ir.clone(), idx: 0 };

    lexer.skip_whitespace();

    // Loop until end of file
    while lexer.idx < (ir.len() - 1).try_into().unwrap() {
      // Test to see if the next token will be letters
      let letters = lexer.tokenize_letters();

      if letters.len() > 0 {
        let tkn = Token::new("$T_LETTERS", &letters);
        lexer.tkns.push(tkn);
        lexer.skip_whitespace();
        continue;
      }

      // Test to see if the next token will be an integer
      let int = lexer.tokenize_integer();

      if int.len() > 0 {
        let tkn = Token::new("$T_INTEGER", &int);
        lexer.tkns.push(tkn);
        lexer.skip_whitespace();
        continue;
      }

      // Test to see if there are any other characters that need to be tokenized
      lexer.tokenize_symbol();
      lexer.idx += 1;
    }

    lexer
  }

  /// Gets the tokens from the token stream
  pub fn get_tkns(&self) -> Vec<Token> {
    self.tkns.clone()
  }
}
