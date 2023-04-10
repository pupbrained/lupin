#![allow(dead_code)] // TODO: remove this

/// Turns a TokenMatchResult into a Result<..., ParseError>
macro_rules! parse_error {
  ($tok:expr) => {{
    let tok = $tok;

    tok
      .map_err(Into::<$crate::error::ParseError>::into)?
      .map_err(|(t, kind)| $crate::error::ParseError::NoMatch {
        possible_matches: kind,
        found: t,
      })
  }};
}

macro_rules! of_symbols {
  ($first_symbol:expr $(, $symbol:expr)* $(,)?) => {
    |t| {
      let possibilities = &[$first_symbol $(, $symbol)*];

      match t.data() {
        lupin_lexer::token::TokenData::Symbol { symbol } if possibilities.contains(symbol) => std::result::Result::Ok(t),
        lupin_lexer::token::TokenData::Symbol { symbol } => std::result::Result::Err($crate::error::ParseError::UnexpectedTokenData {
          mismatch: $crate::error::TokenDataMismatch::Symbol {
            expected_symbols: possibilities.to_vec(),
            found: *symbol,
          },
          span: t.span(),
        }),
        _ => unreachable!(),
      }
    }
  };
}

use {
  crate::error::{ParseError, Result, TokenDataMismatch},
  lupin_lexer::{Symbol, Token, TokenData, TokenKind, Tokens},
};

pub trait Node: Sized {
  fn parse(toks: &mut Tokens) -> Result<Self>;
}

#[derive(Debug)]
pub struct Ast {
  stmt: FnDef,
}

impl Node for Ast {
  fn parse(toks: &mut Tokens) -> Result<Self> {
    let stmt = FnDef::parse(toks)?;

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
    let name_ident = parse_error!(toks.expect(&[TokenKind::Identifier]))?;
    let assign_symbol =
      parse_error!(toks.expect_with(&[TokenKind::Symbol], of_symbols!(Symbol::Assign)))??;
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
pub struct Parenthesized<T> {
  lparen: Token,
  inner: T,
  rparen: Token,
}

impl<T: Node> Node for Parenthesized<T> {
  fn parse(toks: &mut Tokens) -> Result<Self> {
    let lparen =
      parse_error!(toks.expect_with(&[TokenKind::Symbol], of_symbols!(Symbol::LParen)))??;
    let inner = T::parse(toks)?;
    let rparen =
      parse_error!(toks.expect_with(&[TokenKind::Symbol], of_symbols!(Symbol::RParen)))??;

    Ok(Parenthesized {
      lparen,
      inner,
      rparen,
    })
  }
}

impl<T> Parenthesized<T> {
  pub const fn new(lparen: Token, inner: T, rparen: Token) -> Self {
    Self {
      lparen,
      inner,
      rparen,
    }
  }
}

#[derive(Debug)]
pub enum Value {
  Literal(Token),
}

#[derive(Debug)]
pub enum Expression {
  BinaryOp {
    lhs: Box<Expression>,
    symbol: Token,
    rhs: Box<Expression>,
  },

  UnaryOp {
    symbol: Token,
    expr: Box<Expression>,
  },

  Value {
    val: Value,
  },

  Parenthesized {
    parenthesized: Parenthesized<Box<Expression>>,
  },
}

impl Node for Expression {
  fn parse(toks: &mut Tokens) -> Result<Self> {
    let possibilities = &[TokenKind::Symbol, TokenKind::Literal, TokenKind::Identifier];
    let tok = parse_error!(toks.expect(possibilities))?;

    Ok(match tok.data() {
      TokenData::Symbol { symbol } => match symbol {
        Symbol::LParen => {
          // start parsing new inner expression
          let inner_expr = Expression::parse(toks)?;

          // closing rparen
          let rparen =
            parse_error!(toks.expect_with(&[TokenKind::Symbol], of_symbols!(Symbol::RParen)))??;

          let parenthesized = Parenthesized::new(tok, Box::new(inner_expr), rparen);

          Expression::Parenthesized { parenthesized }
        }

        _ => todo!(),
      },

      TokenData::Literal { .. } => Expression::Value {
        val: Value::Literal(tok),
      },

      _ => unreachable!("token thought as matching was unrightfully handled"),
    })
  }
}

#[derive(Debug)]
pub struct Punctuated<T: Node> {
  pairs: Vec<(Result<T>, Option<Token>)>,
}

impl<T: Node> Punctuated<T> {
  pub const fn new(pairs: Vec<(Result<T>, Option<Token>)>) -> Self {
    Self { pairs }
  }
}

impl<T: Node> Node for Punctuated<T> {
  fn parse(toks: &mut Tokens) -> Result<Self> {
    let mut items = Vec::new();
    let mut commas = Vec::new();

    loop {
      let item_result = T::parse(toks);

      items.push(item_result);

      let result_comma =
        parse_error!(toks.expect_with(&[TokenKind::Symbol], of_symbols!(Symbol::Comma)))?;

      match result_comma {
        Ok(comma) => commas.push(comma),
        Err(_) => break,
      }
    }

    let pairs = items
      .into_iter()
      .enumerate()
      .map(|(i, e)| (e, commas.get(i).cloned()))
      .collect::<Vec<(Result<T>, Option<Token>)>>();

    Ok(Punctuated::new(pairs))
  }
}

#[derive(Debug)]
pub struct FnArg {
  ty: Type,
  name: Token,
}

impl Node for FnArg {
  fn parse(toks: &mut Tokens) -> Result<Self> {
    let ty = Type::parse(toks)?;
    let name = parse_error!(toks.expect(&[TokenKind::Identifier]))?;

    Ok(FnArg { ty, name })
  }
}

#[derive(Debug)]
pub struct FnDef {
  ret_ty: Type,
  name: Token,
  two_colons: Token,
  arg_list: Parenthesized<Punctuated<FnArg>>,
}

impl Node for FnDef {
  fn parse(toks: &mut Tokens) -> Result<Self> {
    let ret_ty = Type::parse(toks)?;
    let name = parse_error!(toks.expect(&[TokenKind::Identifier]))?;
    let two_colons =
      parse_error!(toks.expect_with(&[TokenKind::Symbol], of_symbols!(Symbol::TwoColons)))??;
    let arg_list = Parenthesized::<Punctuated<FnArg>>::parse(toks)?;

    Ok(FnDef {
      ret_ty,
      name,
      two_colons,
      arg_list,
    })
  }
}

#[derive(Debug)]
pub struct Type {
  ident: Token,
}

impl Node for Type {
  fn parse(toks: &mut Tokens) -> Result<Self> {
    let ident = parse_error!(toks.expect(&[TokenKind::Identifier]))?;

    Ok(Type { ident })
  }
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
