use logos::{Logos, Span};

pub enum TokenizerErrorKind {
  UnknownToken,
}

pub struct TokenizerError {
  kind: TokenizerErrorKind,
  span: Span,
  slice: String,
}

impl TokenizerError {
  const fn new(kind: TokenizerErrorKind, span: Span, slice: String) -> Self {
    Self { kind, span, slice }
  }
}

pub type Result<T> = core::result::Result<T, TokenizerError>;

#[derive(Logos)]
pub enum Atom {
  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
  Identifier,

  #[token("=")]
  Assign,

  #[regex("(0[bB][01_]+|0[xX][0-9a-fA-F_]+|[0-9][0-9_]*)")]
  Integer,

  #[error]
  #[regex(r"[ \n\t\f]+", logos::skip)]
  Unknown,
}

impl Atom {
  /// Turns an `Atom` into a `Token`.
  ///
  /// Returns `None` if the atom is of variant `Unknown`.
  fn to_token(&self, slice: &str) -> Option<Token> {
    match self {
      Self::Identifier => Some(Token::identifier(slice)),
      Self::Integer => Some(Token::literal(Literal::Integer(slice.to_owned()))),
      Self::Assign => Some(Token::symbol(Symbol::Assign)),
      Self::Unknown => todo!(),
    }
  }
}

pub enum Symbol {
  Assign,
}

pub enum Literal {
  Integer(String),
}

pub enum TokenData {
  Identifier { value: String },

  Symbol { symbol: Symbol },

  Literal { literal: Literal },

  Eof,
}

pub enum TokenKind {
  Identifier,
  Symbol,
  Literal,
  Eof,
}

pub struct Token {
  data: TokenData,
  kind: TokenKind,
}

impl Token {
  fn identifier(value: &str) -> Self {
    Self {
      kind: TokenKind::Identifier,
      data: TokenData::Identifier {
        value: value.to_owned(),
      },
    }
  }

  const fn literal(literal: Literal) -> Self {
    Self {
      kind: TokenKind::Literal,
      data: TokenData::Literal { literal },
    }
  }

  const fn symbol(symbol: Symbol) -> Self {
    Self {
      kind: TokenKind::Symbol,
      data: TokenData::Symbol { symbol },
    }
  }

  const fn eof() -> Self {
    Self {
      kind: TokenKind::Eof,
      data: TokenData::Eof,
    }
  }
}

pub fn tokenize(mut lexer: logos::Lexer<Atom>) -> self::Result<Vec<Token>> {
  use TokenizerErrorKind::UnknownToken;

  let mut tokens = Vec::new();

  while let Some(atom) = lexer.next() {
    let slc = lexer.slice();

    match atom.to_token(slc) {
      None => {
        return Err(TokenizerError::new(
          UnknownToken,
          lexer.span(),
          slc.to_owned(),
        ))
      }
      Some(tok) => tokens.push(tok),
    }
  }

  tokens.push(Token::eof());

  Ok(tokens)
}
