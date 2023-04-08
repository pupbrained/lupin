use {
  lupin_lexer::{token, Lexer},
  std::ops::Range,
};

trait Node: Sized {
  fn parse(lexer: &mut Lexer) -> Self;
}

struct Ast {
  program: Block,
  span: Range<usize>,
}

impl Ast {
  fn from_lexer(mut lexer: Lexer) -> Self {
    let program = Block::parse(&mut lexer);
    todo!()
  }
}

struct Block {
  statements: Vec<Statement>,
  span: Range<usize>,
}

impl Node for Block {
  fn parse(lexer: &mut Lexer) -> Self {
    let first_statement = Statement::parse(lexer);
    todo!()
  }
}

enum Value {
  Name(token::Identifier),
}

pub enum Expression {
  Value(Value),
}

/// just a variable assignment for now
struct Statement {
  ty: Type,
  name: token::Identifier,
  eq_token: token::Assign,
  value: Expression,
}

impl Node for Statement {
  fn parse(lexer: &mut Lexer) -> Self {
    let ty = Type::parse(lexer);
    todo!()
  }
}

struct Type {
  ty: token::Type,
}

impl Node for Type {
  fn parse(lexer: &mut Lexer) -> Self {
    todo!()
  }
}
