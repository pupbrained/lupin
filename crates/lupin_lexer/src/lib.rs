#![feature(assert_matches)]
use logos::{Lexer, Logos};

#[cfg(test)]
mod tests;
pub mod token;

// TODO: move test candidates to repo's root

pub fn lexer(content: &str) -> Lexer<token::Token> {
  token::Token::lexer(content)
}
