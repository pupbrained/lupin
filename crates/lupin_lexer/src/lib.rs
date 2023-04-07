#![feature(assert_matches, let_chains)]
#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]

use std::{iter::Peekable, ops::Range};

#[cfg(test)]
mod tests;
pub mod token;

// derives Logos
// no data attached to the variants
// data instead obtained with the `slice` method of Lexer
enum Atom {
  Identifier,
  Eq,
  Ne,
  Lt,
}

fn extract_tokens(mut lexer: logos::Lexer<Atom>) -> Vec<TokenIdentity> {
  let mut tokens = Vec::new(); // with_capacity size_hint?

  while let Some(atom) = lexer.next() {
    tokens.push(atom.token_identity());
  }

  tokens.push(/* some kind of eof */)
  tokens
}

impl Atom {
  fn token_identity(&self, slice: &str) -> TokenIdentity {
    match self {
      Self::Identifier => TokenIdentity {
        token_type: TokenType::Identifier,
        data: TokenData::Identifier {
          value: slice.to_owned(),
        },
      },

      Self::Eq => TokenIdentity {
        token_type: TokenType::Symbol,
        data: TokenData::Symbol { symbol: Symbol::Eq },
      },

      Self::Ne => TokenIdentity {
        token_type: TokenType::Symbol,
        data: TokenData::Symbol { symbol: Symbol::Ne },
      },

      Self::Lt => TokenIdentity {
        token_type: TokenType::Symbol,
        data: TokenData::Symbol { symbol: Symbol::Lt },
      },
    }
  }
}

enum Symbol {
  Eq,
  Ne,
  Lt,
}

enum TokenType {
  Identifier,
  Symbol,
}

enum TokenData {
  Identifier { value: String },
  Symbol { symbol: Symbol },
}

struct TokenIdentity {
  token_type: TokenType,
  data: TokenData,
}

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
