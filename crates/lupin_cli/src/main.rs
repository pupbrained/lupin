#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]

fn main() -> lupin_parser::Result<()> {
  let program = r"i32 foobar = (1 + (hello + (world)) + (((((woahhhh)))) + 567))";
  let tokens = lupin_lexer::token::tokenize(program);
  let assignment = lupin_parser::parse(tokens);

  println!("{assignment:#?}");

  Ok(())
}
