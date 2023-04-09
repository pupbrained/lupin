#![warn(clippy::unwrap_used)]

pub use error::Result;
use ast::Node;

pub fn create_ast(content: &str) -> crate::Result<ast::Ast> {
  let mut tokens = lupin_lexer::tokenize(content);
  ast::Ast::parse(&mut tokens)
}

pub mod ast;
pub mod error;
