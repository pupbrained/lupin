use {
  crate::error::{Expectation, ParseError},
  lupin_lexer::{Span, Symbol, Token, TokenKind},
  std::ops::Deref,
};

pub struct LazyToken<'a> {
  token: &'a Token,
}

impl Deref for LazyToken<'_> {
  type Target = Token;

  fn deref(&self) -> &Self::Target {
    self.token
  }
}

pub(crate) struct ParserState {
  idx: usize,
  tokens: Vec<Token>,
}

impl ParserState {
  pub fn new(tokens: Vec<Token>) -> Self {
    Self { idx: 0, tokens }
  }

  pub fn peek(&mut self) -> LazyToken {
    let Some(token) = self.tokens.get(self.idx) else {
      panic!("exhausted token buffer; no rules should expect EOF")
    };

    LazyToken { token }
  }

  pub fn take_identifier(&mut self) -> crate::Result<Token> {
    let tok = self.peek();

    if tok.kind() == TokenKind::Identifier {
      self.advance();
      Ok(tok.to_owned())
    } else {
      Err(ParseError {
        found: tok.to_owned(),
        expectation: Expectation::Token(vec![TokenKind::Identifier]),
      })
    }
  }

  pub fn take_symbol(&mut self, symbol: Symbol) -> crate::Result<Token> {
    let tok = self.peek();

    if tok.kind() == TokenKind::Symbol && tok.as_symbol() == symbol {
      self.advance();
      Ok(tok.to_owned())
    } else {
      Err(ParseError {
        found: tok.to_owned(),
        expectation: Expectation::Symbol(vec![symbol]),
      })
    }
  }

  pub fn advance(&mut self) {
    self.idx += 1;
  }
}
