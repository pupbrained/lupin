use {
  crate::atom::{Atom, Symbol},
  logos::Logos,
  std::ops::Range,
};

/// The start and end index of a token.
pub type Span = Range<usize>;

/// Represents the category of a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum TokenKind {
  /// A symbol, usually operators, such as `+`, `-`, `=`, `:=`, `?`, etc.
  Symbol,
  /// A name a value can intake, such as `foo` or `bar2`.
  Identifier,
  /// A literal value, like a number or string.
  Literal,
  /// The end of the file.
  Eof,
  /// An unknown token, of which no known rules could match against, such
  /// as `%`, which is not part of Lupin's syntax.
  Unknown,
}

/// Data associated with a certain token. Can be either an Atom (most
/// common, for all 'normal' tokens), a string containing an unknown
/// token sequence, or no data at all.
#[derive(Debug, Clone)]
pub(crate) enum TokenData {
  Atom(Atom),
  FromUnknown(String),
  None,
}

/// A spanned token with its data.
#[derive(Debug, Clone)]
pub struct Token {
  /// The kind of token represented by `atom`, or `Eof`.
  kind: TokenKind,
  /// The data associated to this token, most often being an atom. See `TokenData`.
  data: TokenData,
  /// The span of the token.
  span: Span,
}

impl Token {
  /// Returns a token with a `TokenKind` of `Identifier`.
  pub(crate) const fn identifier(atom: Atom, span: Span) -> Self {
    Self {
      kind: TokenKind::Identifier,
      data: TokenData::Atom(atom),
      span,
    }
  }

  /// Returns a token with a `TokenKind` of `Symbol`.
  pub(crate) const fn symbol(atom: Atom, span: Span) -> Self {
    Self {
      kind: TokenKind::Symbol,
      data: TokenData::Atom(atom),
      span,
    }
  }

  /// Returns a token with a `TokenKind` of `Literal`.
  pub(crate) const fn literal(atom: Atom, span: Span) -> Self {
    Self {
      kind: TokenKind::Literal,
      data: TokenData::Atom(atom),
      span,
    }
  }

  /// Returns an EOF token.
  pub(crate) const fn eof(span: Span) -> Self {
    Self {
      kind: TokenKind::Eof,
      data: TokenData::None,
      span,
    }
  }

  /// Returns an unknown token
  #[must_use]
  pub(crate) const fn unknown(span: Span, sl: String) -> Self {
    Self {
      kind: TokenKind::Unknown,
      data: TokenData::FromUnknown(sl),
      span,
    }
  }

  /// Returns the token's kind.
  #[must_use]
  pub const fn kind(&self) -> TokenKind {
    self.kind
  }

  /// Returns the token's span.
  #[must_use]
  pub fn span(&self) -> Span {
    self.span.clone()
  }

  /// Returns the token data as a `Symbol`.
  ///
  /// # Panics
  ///
  /// This method will panic if the token's type is not
  /// of a symbol. Ensure `token.kind()` returns `TokenKind::Symbol`
  /// before using this method.
  #[must_use]
  pub fn as_symbol(&self, symbol: Symbol) -> Option<&Token> {
    if let TokenData::Atom(atom) = &self.data {
      atom.symbol().filter(|s| s == &symbol).replace(self)
    } else {
      None
    }
  }
}

/// Tokenizes a string into a collection of `Token`s
#[must_use]
pub fn tokenize(content: &str) -> Vec<Token> {
  let mut atoms = Atom::lexer(content);
  let mut tokens = Vec::new();

  while let Some(tok_result) = atoms.next() {
    let span = atoms.span();

    let tok = match tok_result {
      Ok(tok) => tok.into_token(span),
      Err(()) => Token::unknown(span, atoms.slice().to_owned()),
    };

    tokens.push(tok);
  }

  let last = atoms.span().end;
  tokens.push(Token::eof(last..last));

  tokens
}
