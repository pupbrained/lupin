use logos::{Logos, Lexer};
use crate::token::{Token, Span};

fn as_string(lex: &mut Lexer<Atom>) -> Option<String> {
  lex.slice().parse().ok()
}

/// A symbol, such as `+`, `-`, `:=`, `?`, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
  /// The assignment symbol (`=`).
  Assign,
  /// A comma (`,`).
  Comma,
  /// An opening, or left, parenthese (`(`).
  LParen,
  /// A closing, or right, parenthese (`)`).
  RParen,
  /// The plus sign (`+`)
  Plus,
  /// Two colons, turbofish (`::`)
  TwoColons
}

impl Symbol {
  pub fn is_binop(&self) -> bool {
    use Symbol::*;
    matches!(self, Plus)
  }
}

#[derive(Logos, Debug, Clone)]
#[logos(skip r"[ \n\t\f]+")]
pub(crate) enum Atom {
  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", as_string)]
  Identifier(String),

  #[token("=")]
  Assign,
  #[token(",")]
  Comma,
  #[token("(")]
  LParen,
  #[token(")")]
  RParen,
  #[token("+")]
  Plus,
  #[token("::")]
  TwoColons,

  #[regex("(0[bB][01_]+|0[xX][0-9a-fA-F_]+|[0-9][0-9_]*)", as_string)]
  Integer(String),
}

macro_rules! build_token_match {
  {
    self = $self_param:ident,
    span = $span_param:ident,
    symbols({ $($symb:ident,)+ }),
    literals({ $($lit:ident,)+ }),
  } => {
    let self_ = $self_param;
    let span_ = $span_param;
    match self_ {
      $( Atom::$symb => Token::symbol(self_, span_), )+
      $( Atom::$lit(_) => Token::literal(self_, span_), )+
      Atom::Identifier(_) => Token::identifier(self_, span_),
    }
  }
}

impl Atom {
  /// Infallibly turns the atom into a `Token`
  pub fn into_token(self, span: Span) -> Token {
    build_token_match! {
      self = self,
      span = span,
      symbols({
        Assign,
        Comma,
        LParen,
        RParen,
        Plus,
        TwoColons,
      }),
      literals({
        Integer,
      }),
    }
  }

  /// Returns the kind of symbol represented by the atom.
  ///
  /// # Panics
  ///
  /// This method will panic if the atom does not represent
  /// a symbol.
  pub fn symbol(&self) -> Symbol {
    match self {
      Atom::Assign => Symbol::Assign,
      Atom::Comma => Symbol::Comma,
      Atom::RParen => Symbol::RParen,
      Atom::LParen => Symbol::LParen,
      Atom::Plus => Symbol::Plus,
      Atom::TwoColons => Symbol::TwoColons,
      _ => panic!("atom is NOT a symbol"),
    }
  }
}
