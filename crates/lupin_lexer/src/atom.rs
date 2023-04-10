use {
  crate::{Literal, Symbol, Token, TokenizerError, TokenizerErrorKind},
  logos::{Logos, Span},
  std::borrow::Cow,
};

#[derive(Logos)]
pub(crate) enum Atom {
  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
  Identifier,

  #[token("=")]
  Assign,
  #[token("(")]
  LParen,
  #[token(")")]
  RParen,
  #[token("::")]
  TwoColons,
  #[token(",")]
  Comma,

  #[regex("(0[bB][01_]+|0[xX][0-9a-fA-F_]+|[0-9][0-9_]*)")]
  Integer,

  #[error]
  #[regex(r"[ \n\t\f]+", logos::skip)]
  Unknown,
}

impl Atom {
  /// Turns an `Atom` into a `Token`.
  ///
  /// Returns `TokenizerError::UnknownToken` if the atom is of variant `Unknown`.
  pub fn to_token(&self, slice: Cow<str>, span: Span) -> crate::Result<Token> {
    match self {
      Self::Identifier => Ok(Token::identifier(slice.into_owned(), span)),
      Self::Integer => Ok(Token::literal(Literal::Integer(slice.into_owned()), span)),
      Self::Assign => Ok(Token::symbol(Symbol::Assign, span)),
      Self::LParen => Ok(Token::symbol(Symbol::LParen, span)),
      Self::RParen => Ok(Token::symbol(Symbol::RParen, span)),
      Self::TwoColons => Ok(Token::symbol(Symbol::TwoColons, span)),
      Self::Comma => Ok(Token::symbol(Symbol::Comma, span)),
      Self::Unknown => Err(TokenizerError::new(
        TokenizerErrorKind::UnknownToken,
        span,
        slice.into_owned(),
      )),
    }
  }
}
