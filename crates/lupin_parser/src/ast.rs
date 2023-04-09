#![allow(dead_code)] // TODO: remove this

use {
  crate::error::{ParseError, Result, TokenDataMismatch},
  lupin_lexer::{Symbol, Token, TokenData, TokenKind, Tokens},
};

pub trait Node: Sized {
  fn parse(toks: &mut Tokens) -> Result<Self>;
}

#[derive(Debug)]
pub struct Ast {
  stmt: Statement,
}

impl Node for Ast {
  fn parse(toks: &mut Tokens) -> Result<Self> {
    let stmt = Statement::parse(toks)?;

    Ok(Ast { stmt })
  }
}

#[derive(Debug)]
pub struct Statement {
  ty: Type,
  name_ident: Token,
  assign_symbol: Token,
  value_expr: Expression,
}

impl Node for Statement {
  fn parse(toks: &mut Tokens) -> Result<Self> {
    let ty = Type::parse(toks)?;
    let name_ident = expect_tok(toks, TokenKind::Identifier)?;
    let next_symbol = expect_tok(toks, TokenKind::Symbol)?;
    let assign_symbol = expect_symbol(next_symbol, &[Symbol::Assign])?;
    let value_expr = Expression::parse(toks)?;

    Ok(Statement {
      ty,
      name_ident,
      assign_symbol,
      value_expr,
    })
  }
}

#[derive(Debug)]
pub struct Expression {
  literal: Token,
}

impl Node for Expression {
  fn parse(toks: &mut Tokens) -> Result<Self> {
    let literal = expect_tok(toks, TokenKind::Literal)?;

    Ok(Expression { literal })
  }
}

#[derive(Debug)]
pub struct Type {
  ident: Token,
}

impl Node for Type {
  fn parse(toks: &mut Tokens) -> Result<Self> {
    let ident = expect_tok(toks, TokenKind::Identifier)?;

    Ok(Type { ident })
  }
}

fn expect_tok(toks: &mut Tokens, kind: TokenKind) -> Result<Token> {
  toks
    .expect(kind)
    .map_err(Into::<ParseError>::into)?
    .map_err(|tok| ParseError::UnexpectedTokenKind {
      expected: kind,
      found: tok,
    })
}

/// # Panic
/// Panics if the given token isn't a Symbol.
fn expect_symbol(tok: Token, matches: &[Symbol]) -> Result<Token> {
  if let TokenData::Symbol { symbol } = tok.data() {
    if matches.contains(symbol) {
      Ok(tok)
    } else {
      Err(ParseError::UnexpectedTokenData {
        mismatch: TokenDataMismatch::Symbol {
          expected_symbols: matches.to_owned(),
          found: *symbol,
        },
        span: tok.span(),
      })
    }
  } else {
    panic!("interal error: token is not a symbol");
  }
}
