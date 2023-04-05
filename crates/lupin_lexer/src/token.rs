use {logos::Logos, std::fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub enum IntRep {
  Binary(String),
  Hexadecimal(String),
  Decimal(String),
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
  // Numerical values
  #[regex("(0[bB][01_]+|0[xX][0-9a-fA-F_]+|[0-9][0-9_]*)", |lex| if let Some(s) = lex.slice().get(0..2) {match s {
    "0b" => IntRep::Binary(lex.slice().get(2..).unwrap().replace('_', "")),
    "0x" => IntRep::Hexadecimal(lex.slice().get(2..).unwrap().replace('_', "")),
    _ => IntRep::Decimal(lex.slice().replace('_', ""))
  }} else {IntRep::Decimal(lex.slice().replace('_', ""))})]
  Integer(IntRep),
  #[regex(r"(?:[0-9][0-9_]*\.[0-9_]*|([0-9][0-9_]*)?\.[0-9_]+)", |lex| lex.slice().parse())]
  Float(String), // Exclusively matches string which are guaranteed to be floating-point numbers

  // Strings
  #[regex(r#""([^"\\]|\\t|\\u|\\n|\\|\\r|\\"|\\'")*""#, |lex| lex.slice().parse::<String>().unwrap()[1..lex.slice().len() - 1].parse())]
  String(String),

  // Booleans
  #[regex("true|false", |lex| lex.slice().parse())]
  Boolean(bool),

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
  #[regex("i32|u32|f32|i64|u64|f64|isize|usize|bool|str|char|var", |lex| lex.slice().parse())]
  Type(String),
  #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().parse())]
  Identifier(String),

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

impl Display for IntRep {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      IntRep::Binary(s) => write!(f, "{s}"),
      IntRep::Hexadecimal(s) => write!(f, "{s}"),
      IntRep::Decimal(s) => write!(f, "{s}"),
    }
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Token::Assign => write!(f, ":="),
      Token::AssignMut => write!(f, "="),
      Token::FnDef => write!(f, "::"),
      Token::Identifier(s) => write!(f, "{s}"),
      Token::String(s) => write!(f, "{s}"),
      Token::Integer(i) => write!(f, "{i}"),
      Token::Float(v) => write!(f, "{v}"),
      Token::Boolean(b) => write!(f, "{b}"),
      Token::Type(s) => write!(f, "{s}"),
      Token::If => write!(f, "if"),
      Token::Else => write!(f, "else"),
      Token::ElseIf => write!(f, "elif"),
      Token::Match => write!(f, "match"),
      Token::While => write!(f, "while"),
      Token::For => write!(f, "for"),
      Token::In => write!(f, "in"),
      Token::Loop => write!(f, "loop"),
      Token::Break => write!(f, "break"),
      Token::Gt => write!(f, ">"),
      Token::Lt => write!(f, "<"),
      Token::Ge => write!(f, ">="),
      Token::Le => write!(f, "<="),
      Token::Eq => write!(f, "=="),
      Token::Ne => write!(f, "!="),
      Token::Plus => write!(f, "+"),
      Token::Minus => write!(f, "-"),
      Token::Mult => write!(f, "*"),
      Token::Div => write!(f, "/"),
      Token::FatArrow => write!(f, "=>"),
      Token::LParen => write!(f, "("),
      Token::RParen => write!(f, ")"),
      Token::LBrace => write!(f, "{{"),
      Token::RBrace => write!(f, "}}"),
      Token::LBracket => write!(f, "["),
      Token::RBracket => write!(f, "]"),
      Token::Comma => write!(f, ","),
      Token::Dot => write!(f, "."),
      Token::Return => write!(f, "return"),
      Token::Unknown => write!(f, "???"),
    }
  }
}
