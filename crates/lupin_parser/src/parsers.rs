use crate::parser_state::ParserState;
use crate::error::{Expectation, ParseError};
use lupin_lexer::{TokenKind, Token, Symbol};
use std::marker::PhantomData;

macro_rules! delimiter_impl {
  ($name:ident, $starting_delim:expr, $ending_delim:expr) => {
    #[derive(Debug)]
    struct $name;

    impl self::Delimiter for $name {
      fn starting_delimiter() -> Symbol { $starting_delim }
      fn ending_delimiter() -> Symbol { $ending_delim }
    }
  }
}

/// Matches against the first token of an AST node. The distinction
/// between 'first' token and all other tokens of an AST node is
/// important; in case of the token not matching, the error yielded
/// by this function will contain an `Expectation` of variant `Node`,
/// as opposed to `Token`, in order to provide more helpful and less
/// messy error messages.
fn expect_first_token<T: Node>(state: &mut ParserState, kinds: &[TokenKind]) -> crate::Result<Token> {
  let token = state.peek_locked();
  if kinds.contains(&token.kind()) {
    Ok(token.release_ok())
  } else {
    Err(ParseError {
      found: token.release_err(),
      expectation: Expectation::Node(T::name()),
    })
  }
}

fn expect_token(
  state: &mut ParserState,
  kind: TokenKind,
) -> crate::Result<Token> {
  check_token(state, kind).ok_or_else(|| {
    let found = state.get_elem().clone();

    ParseError {
      found,
      expectation: Expectation::Token(kind),
    }
  })
}

fn check_token(state: &mut ParserState, kind: TokenKind) -> Option<Token> {
  let token = state.peek_locked();

  if token.kind() == kind {
    Some(token.release_ok())
  } else {
    None
  }
}

fn expect_symbol(
  state: &mut ParserState,
  symbol: Symbol,
) -> crate::Result<Token> {
  check_symbol(state, symbol).ok_or_else(|| {
    let found = state.get_elem().clone();

    ParseError {
      found,
      expectation: Expectation::Symbol(symbol),
    }
  })
}

/// Same thing as `expect_symbol`, but doesn't build an error
/// if the token isn't found, just return None.
fn check_symbol(state: &mut ParserState, symbol: Symbol) -> Option<Token> {
  let token = state.peek_locked();

  if token.kind() == TokenKind::Symbol && token.as_symbol() == symbol {
    Some(token.release_ok())
  } else {
    None
  }
}

trait Delimiter {
  fn starting_delimiter() -> Symbol;
  fn ending_delimiter() -> Symbol;
}

delimiter_impl!(ParenDelimiter, Symbol::LParen, Symbol::RParen);

pub(crate) trait Node: Sized {
  fn name() -> &'static str;
  fn parse(state: &mut ParserState) -> crate::Result<Self>;
}

#[derive(Debug)]
struct Type {
  name: Token,
}

impl Node for Type {
  fn name() -> &'static str {
    "type"
  }

  fn parse(state: &mut ParserState) -> crate::Result<Self> {
    let name = expect_first_token::<Self>(state, &[TokenKind::Identifier])?;

    Ok(Self { name })
  }
}

#[derive(Debug)]
enum Expression {
  Parentheses {
    lparen: Token,
    inner_expr: Box<Expression>,
    rparen: Token,
  },

  Literal {
    value: Token,
  },

  Name {
    identifier: Token,
  },

  Binop {
    lhs: Box<Expression>,
    op: Token,
    rhs: Box<Expression>,
  }
}

impl Node for Expression {
  fn name() -> &'static str {
    "expression"
  }

  fn parse(state: &mut ParserState) -> crate::Result<Self> {
    let first_tok = expect_first_token::<Self>(state, &[
      TokenKind::Symbol,
      TokenKind::Identifier,
      TokenKind::Literal,
    ])?;

    fn post_expr_parse<F>(
      state: &mut ParserState,
      as_self_expr: F,
    ) -> crate::Result<Expression>
    where
      F: FnOnce() -> Expression
    { 
      // now, this might be a binop, rather than just a value
      match check_token(state, TokenKind::Symbol) {
        Some(op_tok) if op_tok.as_symbol().is_binop() => {
          let rhs = Expression::parse(state)?;
          
          Ok(Expression::Binop {
            lhs: Box::new(as_self_expr()),
            op: op_tok,
            rhs: Box::new(rhs),
          })
        }

        other => {
          // check_token will release_ok the locked token,
          // therefore advancing the parser state, if
          // it finds any symbol, even if we're not interested
          // in this symbol, we then need to backtrack if
          // it did match. if the token didn't match (its not
          // a symbol), then the parser state was not advanced
          // and we therefore do not need to backtrack.
          if other.is_some() {
            state.backtrack()
          }

          Ok(as_self_expr())
        }
      }
    }

    match first_tok.kind() {
      TokenKind::Symbol => {
        match first_tok.as_symbol() {
          Symbol::LParen => {
            // so it's a parenthesized expression.....

            // lets match on the inner expression
            let inner_expr = Expression::parse(state)?;
            // then the right parenthese!
            let rparen = expect_symbol(state, Symbol::RParen)?;

            // oh but we're not done here. what if the expression
            // is actually a binop? there might be a binop after
            // the rparen.

            post_expr_parse(state, || Expression::Parentheses {
              lparen: first_tok,
              inner_expr: Box::new(inner_expr),
              rparen,
            })
          },

          _ => Err(ParseError {
            found: first_tok,
            expectation: Expectation::Symbol(Symbol::LParen),
          })
        }
      },

      TokenKind::Identifier => {
        post_expr_parse(state, || Expression::Name { identifier: first_tok })
      },

      TokenKind::Literal => {
        post_expr_parse(state, || Expression::Literal { value: first_tok })
      },

      _ => unreachable!(),
    }
  }
}

#[derive(Debug)]
pub struct Assignment {
  ty: Type,
  name: Token,
  assign: Token,
  value: Expression,
}

impl Node for Assignment {
  fn name() -> &'static str {
      "assigment"
  }

  fn parse(state: &mut ParserState) -> crate::Result<Self> {
    let ty = Type::parse(state)?;
    let name = expect_token(state, TokenKind::Identifier)?;
    let assign = expect_symbol(state, Symbol::Assign)?;
    let value = Expression::parse(state)?;

    Ok(Self { ty, name, assign, value })
  }
}

#[derive(Debug)]
struct DelimitedPunctuated<T, D>
where 
  T: Node,
  D: Delimiter,
{
  starting_delim: Token,
  pairs: Vec<(T, Option<Token>)>,
  ending_delim: Token,
  _marker: PhantomData<D>,
}

impl<T, D> Node for DelimitedPunctuated<T, D>
where
  T: Node,
  D: Delimiter,
{
  /// The string name of the node.  
  ///   
  /// This name should never be used. Higher-level nodes
  /// should be implemented, like ArgList or ListConstructor.
  fn name() -> &'static str {
    "<DELIMITED-PUNCTUATED-LIST>"
  }

  fn parse(state: &mut ParserState) -> crate::Result<Self> {
    // first, parse the starting token
    // purpusefully not using the first_token 'convention' here since
    // you never want to show a 'expected delimited list' error msg.
    let starting_delim = expect_symbol(state, D::starting_delimiter())?;

    // in a loop, first look for the ending delimiter.
    // if not present, check for a node T
    
    let mut pairs = Vec::new();
    
    let ending_delim = loop {
      // in the case of no items in the punctuated list, or a trailing comma.
      if let Some(ending_delim) = check_symbol(state, D::ending_delimiter()) {
        break ending_delim;
      }

      let node_t = T::parse(state)?;
      let maybe_comma = check_symbol(state, Symbol::Comma);
      let must_end_here = maybe_comma.is_none();


      pairs.push((node_t, maybe_comma));

      if must_end_here {
        // do a last check for the ending delimiter
        break expect_symbol(state, D::ending_delimiter())?;
      }
    };

    Ok(DelimitedPunctuated::new(pairs, starting_delim, ending_delim))
  }
}

impl<T, D> DelimitedPunctuated<T, D>
where
  T: Node,
  D: Delimiter,
{
  fn new(pairs: Vec<(T, Option<Token>)>, starting_delim: Token, ending_delim: Token) -> Self {
    Self {
      starting_delim,
      pairs,
      ending_delim,
      _marker: PhantomData,
    }
  }
}

#[derive(Debug)]
struct SingleFuncArg {
  ty: Type,
  ident: Token,
}

impl Node for SingleFuncArg {
  fn name() -> &'static str {
    "function argument"
  }

  fn parse(state: &mut ParserState) -> crate::Result<Self> {
    let ty = Type::parse(state)?;
    let ident = expect_token(state, TokenKind::Identifier)?;

    Ok(SingleFuncArg { ty, ident })
  }
}

#[derive(Debug)]
struct FuncArgs {
  args: DelimitedPunctuated<SingleFuncArg, ParenDelimiter>,
}

impl Node for FuncArgs {
  fn name() -> &'static str {
    "function arguments"
  }

  fn parse(state: &mut ParserState) -> crate::Result<Self> {
    let args = DelimitedPunctuated::parse(state)?;
    Ok(FuncArgs { args })
  }
}

#[derive(Debug)]
pub struct FuncDef {
  ty: Type,
  ident: Token,
  two_colons: Token,
  args: FuncArgs,
}

impl Node for FuncDef {
  fn name() -> &'static str {
    "function definition"
  }

  fn parse(state: &mut ParserState) -> crate::Result<Self> {
    let ty = Type::parse(state)?;
    let ident = expect_token(state, TokenKind::Identifier)?;
    let two_colons = expect_symbol(state, Symbol::TwoColons)?;
    let args = FuncArgs::parse(state)?;

    Ok(FuncDef { ty, ident, two_colons, args })
  }
}
