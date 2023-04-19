use {
  crate::{
    error::{Expectation, ParseError},
    parser_state::ParserState,
  },
  lupin_lexer::{Symbol, Token, TokenKind},
  std::marker::PhantomData,
};

macro_rules! delimiter_impl {
  ($name:ident, $starting_delim:expr, $ending_delim:expr) => {
    #[derive(Debug)]
    struct $name;

    impl self::Delimiter for $name {
      fn starting_delimiter() -> Symbol {
        $starting_delim
      }

      fn ending_delimiter() -> Symbol {
        $ending_delim
      }
    }
  };
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
    Ok(Self {
      name: state.take_identifier()?,
    })
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
  },
}

impl Node for Expression {
  fn name() -> &'static str {
    "expression"
  }

  fn parse(state: &mut ParserState) -> crate::Result<Self> {
    fn post_expr_parse<F>(state: &mut ParserState, as_self_expr: F) -> crate::Result<Expression>
    where
      F: FnOnce() -> Expression,
    {
      // now, this might be a binop, rather than just a value
      let tok = state.peek();

      if tok.kind() == TokenKind::Symbol && tok.as_symbol().is_binop() {
        state.advance();

        let rhs = Expression::parse(state)?;

        Ok(Expression::Binop {
          lhs: Box::new(as_self_expr()),
          op: tok.to_owned(),
          rhs: Box::new(rhs),
        })
      } else {
        Ok(as_self_expr())
      }
    }

    let first_tok = state.peek();

    match first_tok.kind() {
      TokenKind::Symbol => {
        match first_tok.as_symbol() {
          Symbol::LParen => {
            // so it's a parenthesized expression.....
            state.advance();

            // lets match on the inner expression
            let inner_expr = Expression::parse(state)?;
            // then the right parenthese!
            let rparen = state.take_symbol(Symbol::RParen)?;

            // oh but we're not done here. what if the expression
            // is actually a binop? there might be a binop after
            // the rparen.

            post_expr_parse(state, || Expression::Parentheses {
              lparen: first_tok.to_owned(),
              inner_expr: Box::new(inner_expr),
              rparen,
            })
          }

          _ => Err(ParseError {
            found: first_tok.to_owned(),
            expectation: Expectation::Symbol(vec![Symbol::LParen]),
          }),
        }
      }

      TokenKind::Identifier => {
        state.advance();

        post_expr_parse(state, || Expression::Name {
          identifier: first_tok.to_owned(),
        })
      }

      TokenKind::Literal => {
        state.advance();

        post_expr_parse(state, || Expression::Literal {
          value: first_tok.to_owned(),
        })
      }

      _ => Err(ParseError {
        found: first_tok.to_owned(),
        expectation: Expectation::Token(vec![TokenKind::Symbol, TokenKind::Identifier, TokenKind::Literal]),
      }),
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
    let name = state.take_identifier()?;
    let assign = state.take_symbol(Symbol::Assign)?;
    let value = Expression::parse(state)?;

    Ok(Self {
      ty,
      name,
      assign,
      value,
    })
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
    let starting_delim = state.take_symbol(D::starting_delimiter())?;

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
        break state.take_symbol(D::ending_delimiter())?;
      }
    };

    Ok(DelimitedPunctuated::new(
      pairs,
      starting_delim,
      ending_delim,
    ))
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
    let ident = state.take_identifier()?;

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
    let ident = state.take_identifier()?;
    let two_colons = state.take_symbol(Symbol::TwoColons)?;
    let args = FuncArgs::parse(state)?;

    Ok(FuncDef {
      ty,
      ident,
      two_colons,
      args,
    })
  }
}
