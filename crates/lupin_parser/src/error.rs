use lupin_lexer::{Token, TokenKind, Symbol};

#[derive(Debug)]
pub enum Expectation {
  Token(Vec<TokenKind>),
  Symbol(Vec<Symbol>),
  Node(&'static str),
  Binop,
}

#[derive(Debug)]
pub struct ParseError {
  pub(crate) found: Token,
  pub(crate) expectation: Expectation,
}
