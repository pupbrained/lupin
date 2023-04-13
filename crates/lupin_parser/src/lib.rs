#![warn(clippy::unwrap_used)]

use lupin_lexer::Token;
use parsers::{Assignment, Node};

pub type Result<T> = std::result::Result<T, error::ParseError>;

pub fn parse(tokens: Vec<Token>) -> crate::Result<Assignment> {
  let mut state = parser_state::ParserState::new(tokens);

  Assignment::parse(&mut state)
}

pub mod parser_state;
pub mod error;
pub mod parsers;

