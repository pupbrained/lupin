use {
  crate::token::{Float, Token},
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
  [
    ("1.", Some(Token::Float(Float { value: "1.".into() }))),
    (
      "24_.",
      Some(Token::Float(Float {
        value: "24_.".into(),
      })),
    ),
    (
      "3_49.",
      Some(Token::Float(Float {
        value: "3_49.".into(),
      })),
    ),
    (
      "5.6",
      Some(Token::Float(Float {
        value: "5.6".into(),
      })),
    ),
    (
      "7_8.9",
      Some(Token::Float(Float {
        value: "7_8.9".into(),
      })),
    ),
    (".0", Some(Token::Float(Float { value: ".0".into() }))),
    (
      "._1",
      Some(Token::Float(Float {
        value: "._1".into(),
      })),
    ),
    (
      ".0_12",
      Some(Token::Float(Float {
        value: ".0_12".into(),
      })),
    ),
  ]
  .iter()
  .for_each(|(candidate, token)| assert_eq!(&Token::lexer(candidate).next(), token));
}

#[test]
fn integer_representations() {
  use {
    crate::token::Integer::{Binary, Decimal, Hexadecimal},
    Token::Integer,
  };

  [
    ("100", Some(Integer(Decimal("100".into())))),
    ("0b1011", Some(Integer(Binary("0b1011".into())))),
    ("0b1011_1010", Some(Integer(Binary("0b1011_1010".into())))),
    ("0x1a", Some(Integer(Hexadecimal("0x1a".into())))),
    ("0x1a_2b", Some(Integer(Hexadecimal("0x1a_2b".into())))),
  ]
  .iter()
  .for_each(|(candidate, token)| assert_eq!(&Token::lexer(candidate).next(), token));
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
