#![warn(clippy::unwrap_used)]

use lupin_lexer::token::{Identifier, Type};

pub struct Tree {
  statements: Vec<Statement>,
}

pub enum AssignKind {
  Mutable,
  Immutable,
}

/// a variable assignment
pub struct Statement {
  ty: Type,
  ident: Identifier,
  assign_kind: AssignKind,
  value: Expression,
}

mod old {
  use lupin_lexer::{token::Token, Lexer};

  pub enum ParseError {
    Unexpected(Token),
  }

  pub struct ParseContext<'a> {
    lexer: Lexer<'a>,
  }

  pub struct Statement {
    var_ident: String,
    expr: String,
    ty: String,
  }

  // TODO: Would probably be far more convenient to have structs for
  // types of tokens
  // struct Token {
  //     Float(tok::Float),
  //     String(tok::String),
  //     AssignOp(tok::Assign),
  //     etc
  // }
  pub enum Name {
    Literal(Token),
    Identifier(Token),
  }

  pub enum Operator {
    Plus(Token),
    Minus(Token),
    Mult(Token),
    Div(Token),
  }

  impl<'a> ParseContext<'a> {
    const fn new(lexer: Lexer<'a>) -> Self {
      Self { lexer }
    }

    fn name(&mut self) -> Name {
      let next_tok = self.lexer.next_token();

      if let Some(tok @ Token::Float(_)) = next_tok {
        Name::Literal(tok)
      } else if let Some(tok @ Token::Identifier(_)) = next_tok {
        Name::Identifier(tok)
      } else {
        panic!("expected name (float or identifier)");
      }
    }

    const fn is_operator(tok: &Token) -> bool {
      matches!(tok, Token::Plus | Token::Minus | Token::Mult | Token::Div)
    }

    /// Parses an expression
    /// Expressions are floats
    /// # Panics
    ///
    /// Will panic if the next token is not a float
    pub fn expression(&mut self) -> String {
      // parsing expressions...
      // name ::= <literal>
      // name ::= <identifier>
      // expr ::= <expr> <operator> <expr>
      // expr ::= <name>
      // expr ::= "("? <expr> ")"?

      let _name = self.name();

      if let Some(tok) = self.lexer.peek() {
        if ParseContext::is_operator(tok) 
          let _rhs_expr = self.expression();
        
      }

      String::new()
    }

    /// Parses a statement
    ///
    /// Statements are immutable variable declarations:
    /// <type> <identifier> = <expr>
    /// # Panics
    ///
    /// Will panic if it cannot parse a statement
    pub fn statement(&mut self) -> Statement {
      if let Some(Token::Type(ty)) = self.lexer.next_token() {
        if let Some(Token::Identifier(ident)) = self.lexer.next_token() {
          if let Some(Token::Assign | Token::AssignMut) = self.lexer.next_token() {
            let expr = self.expression();
            Statement {
              var_ident: ident.into(),
              ty: ty.into(),
              expr,
            }
          } else {
            panic!("assign not found");
          }
        } else {
          panic!("identifier not found");
        }
      } else {
        panic!("type not found");
      }
    }
  }

  pub fn parse(content: &str) {
    let lexer = Lexer::new(content);
    let mut ctx = ParseContext::new(lexer);
    let stmt = ctx.statement();
    let Statement {
      ty,
      var_ident,
      expr,
    } = stmt;

    println!("{ty} {var_ident} = {expr}");
  }
}

pub use old::parse;
