use {
  crate::token::Token,
  anyhow::Result,
  logos::Logos,
  std::{
    assert_matches::assert_matches,
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Flatten,
    path::Path,
  },
};

fn read_lines(filename: impl AsRef<Path>) -> Result<Flatten<Lines<BufReader<File>>>> {
  let file = File::open(filename)?;
  Ok(BufReader::new(file).lines().flatten())
}

#[test]
fn floats() {
  use Token::*;

  [
    ("1.", Some(Float("1.".into()))),
    ("24_.", Some(Float("24_.".into()))),
    ("3_49.", Some(Float("3_49.".into()))),
    ("5.6", Some(Float("5.6".into()))),
    ("7_8.9", Some(Float("7_8.9".into()))),
    (".0", Some(Float(".0".into()))),
    ("._1", Some(Float("._1".into()))),
    (".0_12", Some(Float(".0_12".into()))),
  ]
  .iter()
  .for_each(|(candidate, token)| assert_eq!(&Token::lexer(candidate).next(), token));
}

#[test]
fn float_test_new_method() {
  use Token::Float;
  let _candidates = ["1.", "24_.", "3_49.", "5.6", "7_8.9", ".0", "._1", ".0_12"];
  let _tokens = [
    Float("1.".to_owned()),
    Float("24_.".to_owned()),
    Float("3_49.".to_owned()),
    Float("5.6".to_owned()),
    Float("7_8.9".to_owned()),
    Float(".0".to_owned()),
    Float("._1".to_owned()),
    Float(".0_12".to_owned()),
  ];
}

#[test]
fn floating_point_numbers() -> Result<()> {
  read_lines("../../tests/tokenizer-candidates/float-passing.txt")?
    .for_each(|l| assert_matches!(Token::lexer(&l).next(), Some(Token::Float(_))));

  read_lines("../../tests/tokenizer-candidates/float-failing.txt")?
    .for_each(|l| assert!(!matches!(Token::lexer(&l).next(), Some(Token::Float(_)))));

  Ok(())
}

#[test]
fn strings() -> Result<()> {
  read_lines("../../tests/tokenizer-candidates/string-passing.txt")?
    .for_each(|l| assert_matches!(Token::lexer(&l).next(), Some(Token::String(_))));

  Ok(())
}
