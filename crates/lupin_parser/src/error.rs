use lupin_lexer::{Token, TokenKind, Symbol};


#[derive(Debug)]
pub enum Expectation {
  Token(TokenKind),
  Symbol(Symbol),
  Node(&'static str),
  Binop,
}

#[derive(Debug)]
pub struct ParseError {
  pub(crate) found: Token,
  pub(crate) expectation: Expectation,
}
