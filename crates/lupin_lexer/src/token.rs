#![allow(clippy::module_name_repetitions)]

use {logos::Logos, std::fmt::Display};

trait FromLexer: Sized {
  fn from_lexer(lex: &mut logos::Lexer<Token>) -> Self;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Integer {
  Binary(String),
  Hexadecimal(String),
  Decimal(String),
}

impl FromLexer for Integer {
  fn from_lexer(lex: &mut logos::Lexer<Token>) -> Self {
    match lex.slice().get(0..2) {
      Some("0b") => Self::Binary(lex.slice().into()),
      Some("0x") => Self::Hexadecimal(lex.slice().into()),
      _ => Self::Decimal(lex.slice().into()),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Float {
  pub value: String,
}

impl FromLexer for Float {
  fn from_lexer(lex: &mut logos::Lexer<Token>) -> Self {
    Self {
      value: lex.slice().to_owned(),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StringToken {
  value: String,
}

impl FromLexer for StringToken {
  fn from_lexer(lex: &mut logos::Lexer<Token>) -> Self {
    Self {
      value: lex.slice().to_owned(),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Boolean {
  value: bool,
}

impl FromLexer for Boolean {
  fn from_lexer(lex: &mut logos::Lexer<Token>) -> Self {
    Self {
      value: match lex.slice() {
        "true" => true,
        "false" => false,
        _ => unreachable!("regex should always match true or false"),
      },
    }
  }
}

// macro jumpscare
// omg macro is gone?

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct If;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Else;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Elif;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Match;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct While;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct For;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct In;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Loop;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Break;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Assign;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssignMut;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Plus;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Minus;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
  I32,
  I64,
  U32,
  U64,
  F32,
  F64,
  Isize,
  Usize,
  Str,
  Char,
  Var,
}

impl FromLexer for Type {
  fn from_lexer(lex: &mut logos::Lexer<Token>) -> Self {
    use Type::{Char, Isize, Str, Usize, Var, F32, F64, I32, I64, U32};
    match lex.slice() {
      "i32" => I32,
      "i64" => I64,
      "u32" => U32,
      "f32" => F32,
      "f64" => F64,
      "isize" => Isize,
      "usize" => Usize,
      "str" => Str,
      "char" => Char,
      "var" => Var,
      _ => unreachable!("regex should always match a builtin type"),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
  value: String,
}

impl FromLexer for Identifier {
  fn from_lexer(lex: &mut logos::Lexer<Token>) -> Self {
    Self {
      value: lex.slice().to_owned(),
    }
  }
}

#[derive(Logos, Debug, PartialEq, Eq, Clone)]
pub enum Token {
  // Numerical values
  #[regex("(0[bB][01_]+|0[xX][0-9a-fA-F_]+|[0-9][0-9_]*)", Integer::from_lexer)]
  Integer(Integer),

  #[regex(
    r"(?:[0-9][0-9_]*\.[0-9_]*|([0-9][0-9_]*)?\.[0-9_]+)",
    Float::from_lexer
  )]
  Float(Float), // Exclusively matches string which are guaranteed to be floating-point numbers

  // Strings
  #[regex(r#""([^"\\]|\\t|\\u|\\n|\\|\\r|\\"|\\'")*""#, StringToken::from_lexer)]
  String(StringToken),

  // Booleans
  #[regex("true|false", Boolean::from_lexer)]
  Boolean(Boolean),

  // Conditionals
  #[token("if")]
  If,
  #[token("else")]
  Else,
  #[token("elif")]
  ElseIf,
  #[token("match")]
  Match,
  #[token("while")]
  While,
  #[token("for")]
  For,
  #[token("in")]
  In,
  #[token("loop")]
  Loop,
  #[token("break")]
  Break,

  // Identifiers
  #[regex(
    "i32|u32|f32|i64|u64|f64|isize|usize|bool|str|char|var",
    Type::from_lexer
  )]
  Type(Type),
  #[regex("[a-zA-Z_][a-zA-Z0-9_]*", Identifier::from_lexer)]
  Identifier(Identifier),

  // Value setting
  #[token(":=")]
  Assign,
  #[token("=")]
  AssignMut,
  #[token("::")]
  FnDef,

  // Operators "<|>|<=|>=|!=|==|[=+\-*/]"
  #[token(">")]
  Gt,
  #[token("<")]
  Lt,
  #[token(">=")]
  Ge,
  #[token("<=")]
  Le,
  #[token("==")]
  Eq,
  #[token("!=")]
  Ne,
  #[token(",")]
  Comma,
  #[token(".")]
  Dot,
  #[token("+")]
  Plus,
  #[token("-")]
  Minus,
  #[token("*")]
  Mult,
  #[token("/")]
  Div,

  // Control
  #[token("=>")]
  FatArrow,

  // Brackets
  #[token("(")]
  LParen,
  #[token(")")]
  RParen,
  #[token("{")]
  LBrace,
  #[token("}")]
  RBrace,
  #[token("[")]
  LBracket,
  #[token("]")]
  RBracket,

  #[token("return")]
  Return,

  #[error]
  #[regex(r"[ \n\t\f]+", logos::skip)]
  Unknown,
}

impl Token {
  #[must_use] pub const fn token_kind(&self) -> TokenKind {
    use TokenKind::{Identifier, Todo};

    match self {
      Self::Identifier(_) => Identifier,
      _ => Todo,
    }
  }
}

pub enum TokenKind {
  Identifier,
  Todo,
}

impl Display for Integer {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Binary(s) | Self::Hexadecimal(s) | Self::Decimal(s) => write!(f, "{s}"),
    }
  }
}

macro_rules! display_impl {
  ($type:ty) => {
    impl std::fmt::Display for $type {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
      }
    }
  };
}

display_impl!(Float);
display_impl!(Identifier);
display_impl!(StringToken);
display_impl!(Boolean);

impl Display for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::I32 => write!(f, "i32"),
      Self::I64 => write!(f, "i64"),
      Self::U32 => write!(f, "u32"),
      Self::U64 => write!(f, "u64"),
      Self::F32 => write!(f, "f32"),
      Self::F64 => write!(f, "f64"),
      Self::Isize => write!(f, "isize"),
      Self::Usize => write!(f, "usize"),
      Self::Str => write!(f, "str"),
      Self::Char => write!(f, "char"),
      Self::Var => write!(f, "var"),
    }
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Assign => write!(f, ":="),
      Self::AssignMut => write!(f, "="),
      Self::FnDef => write!(f, "::"),
      Self::Identifier(s) => write!(f, "{s}"),
      Self::String(s) => write!(f, "{s}"),
      Self::Type(s) => write!(f, "{s}"),
      Self::Float(v) => write!(f, "{v}"),
      Self::Integer(s) => write!(f, "{s}"),
      Self::Boolean(b) => write!(f, "{b}"),
      Self::If => write!(f, "if"),
      Self::Else => write!(f, "else"),
      Self::ElseIf => write!(f, "elif"),
      Self::Match => write!(f, "match"),
      Self::While => write!(f, "while"),
      Self::For => write!(f, "for"),
      Self::In => write!(f, "in"),
      Self::Loop => write!(f, "loop"),
      Self::Break => write!(f, "break"),
      Self::Gt => write!(f, ">"),
      Self::Lt => write!(f, "<"),
      Self::Ge => write!(f, ">="),
      Self::Le => write!(f, "<="),
      Self::Eq => write!(f, "=="),
      Self::Ne => write!(f, "!="),
      Self::Plus => write!(f, "+"),
      Self::Minus => write!(f, "-"),
      Self::Mult => write!(f, "*"),
      Self::Div => write!(f, "/"),
      Self::FatArrow => write!(f, "=>"),
      Self::LParen => write!(f, "("),
      Self::RParen => write!(f, ")"),
      Self::LBrace => write!(f, "{{"),
      Self::RBrace => write!(f, "}}"),
      Self::LBracket => write!(f, "["),
      Self::RBracket => write!(f, "]"),
      Self::Comma => write!(f, ","),
      Self::Dot => write!(f, "."),
      Self::Return => write!(f, "return"),
      Self::Unknown => write!(f, "???"),
    }
  }
}
