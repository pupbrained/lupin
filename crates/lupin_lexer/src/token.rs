use {
  crate::{atom::Atom, Span},
  std::iter::FusedIterator,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
  Assign,
}

#[derive(Debug)]
pub enum Literal {
  Integer(String),
}

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum TokenData {
  Identifier { value: String },

  Symbol { symbol: Symbol },

  Literal { literal: Literal },

  Eof,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(clippy::module_name_repetitions)]
pub enum TokenKind {
  Identifier,
  Symbol,
  Literal,
  Eof,
}

#[derive(Debug)]
pub struct Token {
  data: TokenData,
  kind: TokenKind,
  span: Span,
}

impl Token {
  #[must_use]
  pub const fn identifier(value: String, span: Span) -> Self {
    Self {
      kind: TokenKind::Identifier,
      data: TokenData::Identifier { value },
      span,
    }
  }

  #[must_use]
  pub const fn literal(literal: Literal, span: Span) -> Self {
    Self {
      kind: TokenKind::Literal,
      data: TokenData::Literal { literal },
      span,
    }
  }

  #[must_use]
  pub const fn symbol(symbol: Symbol, span: Span) -> Self {
    Self {
      kind: TokenKind::Symbol,
      data: TokenData::Symbol { symbol },
      span,
    }
  }

  #[must_use]
  pub const fn eof(span: Span) -> Self {
    Self {
      kind: TokenKind::Eof,
      data: TokenData::Eof,
      span,
    }
  }

  #[must_use]
  pub const fn data(&self) -> &TokenData {
    &self.data
  }

  #[must_use]
  pub const fn kind(&self) -> TokenKind {
    self.kind
  }

  #[must_use]
  pub fn span(&self) -> Span {
    // const-cloning of Ranges is unsupported
    self.span.clone()
  }
}

pub struct Tokens<'s> {
  found_eof: bool,
  lexer: logos::Lexer<'s, Atom>,
}

/// Lazy iterator over Token  
///   
/// Lazily turns `Atom`s into `Token`s, returning `Token::Eof` after the last `Atom`. Returns `None` afterwards.
impl<'s> Tokens<'s> {
  pub(crate) const fn new(lexer: logos::Lexer<'s, Atom>) -> Self {
    Self {
      found_eof: false,
      lexer,
    }
  }

  /// Returns `Result::Ok` with the next token if its kind matches `kind`, `Result::Err` with the next token otherwise.  
  ///   
  /// # Errors
  /// The top-level error is a lexer error, meaning something wrong happened while lexing. The
  /// inner `Result` represents if the `TokenKind` matched as explained above.
  pub fn expect(&mut self, kind: TokenKind) -> crate::Result<Result<Token, Token>> {
    self.next().transpose().map(|mb_tok| {
      // mb_tok should not be a None variant, since EOF must've been handled.
      let tok =
        mb_tok.expect("Some(Eof) handling should prevent iterator from being consumed again");

      if tok.kind == kind {
        Ok(tok)
      } else {
        Err(tok)
      }
    })
  }
}

impl<'a> Iterator for Tokens<'a> {
  type Item = crate::Result<Token>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.found_eof {
      return None;
    }

    Some(match self.lexer.next() {
      Some(atom) => atom.to_token(self.lexer.slice().into(), self.lexer.span()),
      None => {
        self.found_eof = true;
        Ok(Token::eof(self.lexer.span()))
      }
    })
  }
}

impl<'a> FusedIterator for Tokens<'a> {}
