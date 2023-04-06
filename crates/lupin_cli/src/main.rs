#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]

fn main() {
  let program = "f32 a := .3";
  lupin_parser::parse(program);
}
