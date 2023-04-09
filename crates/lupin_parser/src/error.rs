use lupin_lexer::{Span, Symbol, Token, TokenKind, TokenizerError};

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub enum TokenDataMismatch {
  Symbol {
    expected_symbols: Vec<Symbol>,
    found: Symbol,
  },
}

#[derive(Debug)]
pub enum ParseError {
  TokenizeError(TokenizerError),
  UnexpectedTokenKind {
    expected: TokenKind,
    found: Token,
  },
  UnexpectedTokenData {
    mismatch: TokenDataMismatch,
    span: Span,
  },
}

impl From<TokenizerError> for ParseError {
  fn from(value: TokenizerError) -> Self {
    ParseError::TokenizeError(value)
  }
}
