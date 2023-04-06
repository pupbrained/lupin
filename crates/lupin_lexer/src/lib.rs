#![feature(assert_matches)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]

use {
  logos::{Lexer as LogosLexer, Logos},
  std::iter::Peekable,
};

#[cfg(test)]
mod tests;
pub mod token;

// TODO: move test candidates to repo's root

pub struct Lexer<'a> {
  lexer: Peekable<LogosLexer<'a, token::Token>>,
}

impl<'a> Lexer<'a> {
  #[must_use] pub fn new(content: &'a str) -> Self {
    Self {
      lexer: token::Token::lexer(content).peekable(),
    }
  }

  pub fn next_token(&mut self) -> Option<token::Token> {
    self.lexer.next()
  }

  pub fn peek(&mut self) -> Option<&token::Token> {
    self.lexer.peek()
  }
}
