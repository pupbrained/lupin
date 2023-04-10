use {
  crate::{atom::Atom, Span},
  std::iter::{FusedIterator, Peekable},
};

pub type TokenMatchResult<T> = crate::Result<Result<T, (Token, Vec<TokenKind>)>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
  Assign,
  Plus,
  Comma,
  LParen,
  RParen,
  TwoColons,
}

#[derive(Debug, Clone)]
pub enum Literal {
  Integer(String),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

struct TokensInner<'s> {
  found_eof: bool,
  lexer: logos::Lexer<'s, Atom>,
}

impl<'a> Iterator for TokensInner<'a> {
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

impl<'s> FusedIterator for TokensInner<'s> {}

pub struct Tokens<'s> {
  iter: Peekable<TokensInner<'s>>,
}

/// Lazy iterator over Token  
///
/// Lazily turns `Atom`s into `Token`s, returning `Token::Eof` after the last `Atom`. Returns `None` afterwards.
impl<'s> Tokens<'s> {
  pub(crate) fn new(lexer: logos::Lexer<'s, Atom>) -> Self {
    Self {
      iter: TokensInner {
        found_eof: false,
        lexer,
      }
      .peekable(),
    }
  }

  /// Returns `Result::Ok` with the next token if its kind matches `kind`, `Result::Err` with the next token otherwise.  
  ///
  /// # Errors
  /// The top-level error is a lexer error, meaning something wrong happened while lexing. The
  /// inner `Result` represents if the `TokenKind` matched as explained above.
  pub fn expect(&mut self, possibilities: &[TokenKind]) -> TokenMatchResult<Token> {
    self.expect_with(possibilities, std::convert::identity)
  }

  /// # Errors
  /// shut up
  /// # Panics
  /// SHUT UPPPPP :3333
  pub fn expect_with<F, T>(
    &mut self,
    possibilities: &[TokenKind],
    predicate: F,
  ) -> TokenMatchResult<T>
  where
    F: FnOnce(Token) -> T,
  {
    if let Some(result) = self.iter.peek() {
      match result {
        Ok(tok) if possibilities.contains(&tok.kind) => {
          let owned_tok = self
            .iter
            .next()
            .expect("already matched")
            .expect("already matched");

          let result = predicate(owned_tok);

          Ok(Ok(result))
        }
        Ok(tok) => Ok(Err((tok.clone(), possibilities.to_owned()))),
        Err(err) => Err(err.clone()),
      }
    } else {
      panic!("Some(Eof) handling should prevent iterator from being consumed again");
    }
  }
}
