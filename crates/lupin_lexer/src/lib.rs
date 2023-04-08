#![feature(assert_matches, let_chains)]
#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::missing_errors_doc)] // temporary

use std::{iter::Peekable, ops::Range};

pub mod atom;
#[cfg(test)]
mod tests;
pub mod token;

// derives Logos
// no data attached to the variants
// data instead obtained with the `slice` method of Lexer
// TODO: move test candidates to repo's root

pub struct Lexer<'s> {
  iter: Peekable<logos::SpannedIter<'s, token::Token>>,
}

impl<'s> Lexer<'s> {
  #[must_use]
  pub fn new(lexer: logos::Lexer<'s, token::Token>) -> Self {
    Self {
      iter: lexer.spanned().peekable(),
    }
  }

  pub fn next_token(&mut self) -> Option<(token::Token, Range<usize>)> {
    self.iter.next()
  }

  pub fn advance(&mut self) {
    // gay
    let _ = self.next_token();
  }

  // pub fn expect_kind(&mut self, kind: token::TokenKind) -> (token::Token, Range<usize>) {
  //   if let Some((tok, _)) = self.iter.peek() {}
  // }
}
