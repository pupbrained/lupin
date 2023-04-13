#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]

fn main() -> lupin_parser::Result<()> {
  let program = r"i32 super_cool_func :: (i32 okay, u32 wowiwie,)";
  let tokens = lupin_lexer::token::tokenize(program);
  let assignment = lupin_parser::parse(tokens);

  println!("{assignment:#?}");

  Ok(())
}
