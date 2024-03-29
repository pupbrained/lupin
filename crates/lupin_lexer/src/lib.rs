#![feature(assert_matches, let_chains)]
#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::single_match_else)]
#![allow(dead_code)] // TODO: remove this

pub use token::{Token, TokenKind, Span};
pub use atom::Symbol;

pub type Result<T> = std::result::Result<T, TokenizerError>;

#[derive(Debug, Clone, Copy)]
pub enum TokenizerErrorKind {
  UnknownToken,
}

#[derive(Debug, Clone)]
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

pub mod atom;
pub mod token;
