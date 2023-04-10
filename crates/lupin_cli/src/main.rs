#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
// erm.._
// can u send in discord
//

fn test_program(program: &str) -> lupin_parser::error::Result<String> {
  let ast = lupin_parser::create_ast(program)?;
  Ok(format!("PROGRAM: '{program}':\n{ast:#?}\n\n"))
}

fn main() {
  println!("{:#?}", test_program("i32 foobarfn :: (u32 a, i16 b)"));
  println!("{:#?}", test_program("ty fname :: (cooltype 7failure, )"));
  println!("{:#?}", test_program("tyawesome ffffff :: ()"));
  println!("{:#?}", test_program("meow woof :: (OKAYYYYYY walk)"));
  println!("{:#?}", test_program("idontknow ok :: (&&)"));
}
// hi 

