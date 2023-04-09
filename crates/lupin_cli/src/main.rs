#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]

fn main() -> lupin_parser::error::Result<()> {
  let program = "f32 a = 3829";
  let ast = lupin_parser::create_ast(program)?; // so safe
  println!("{ast:#?}");
  Ok(())
}
