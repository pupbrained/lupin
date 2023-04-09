#![feature(assert_matches, let_chains)]
#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::single_match_else)]
#![allow(dead_code)] // TODO: remove this

pub use token::{Literal, Symbol, Token, TokenData, TokenKind, Tokens};
use {logos::Logos, std::ops::Range};

pub type Span = Range<usize>;
pub type Result<T> = std::result::Result<T, TokenizerError>;

#[derive(Debug)]
pub enum TokenizerErrorKind {
  UnknownToken,
}

#[derive(Debug)]
pub struct TokenizerError {
  kind: TokenizerErrorKind,
  span: Span,
  slice: String,
}

impl TokenizerError {
  const fn new(kind: TokenizerErrorKind, span: Span, slice: String) -> Self {
    Self { kind, span, slice }
  }
}

#[must_use]
pub fn tokenize(content: &str) -> Tokens<'_> {
  let lexer = atom::Atom::lexer(content);
  Tokens::new(lexer)
}

pub mod atom;
pub mod token;
