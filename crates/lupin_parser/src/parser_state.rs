use lupin_lexer::{Token, TokenKind, Span, Symbol};

pub struct LockedToken<'a> {
  token: Token,
  state: &'a mut ParserState,
}

impl LockedToken<'_> {
  pub fn release_ok(mut self) -> Token {
    self.state.idx += 1;
    self.token
  }

  pub fn release_err(self) -> Token {
    self.token
  }

  // implement all public methods from Token
  // not using Deref so you can't get a Token
  // from a LockedToken other than by explicitly
  // stating if the token matched (ok) or no (err)

  pub fn kind(&self) -> TokenKind {
    self.token.kind()
  }

  pub fn span(&self) -> Span {
    self.token.span()
  }

  pub fn as_symbol(&self) -> Symbol {
    self.token.as_symbol()
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

  pub fn get_elem(&self) -> &Token {
    let Some(elem) = self.tokens.get(self.idx) else {
      panic!("exhausted token buffer: no rule should expect EOF")
    };

    elem
  }

  pub fn peek_locked(&mut self) -> LockedToken {
    // check to see if the buffer isn't exhausted
    let token = self.get_elem().clone();

    LockedToken {
      state: self,
      token,
    }
  }  

  pub fn backtrack(&mut self) {
    self.idx -= 1;
  }
}
