#![feature(assert_matches, let_chains)]
#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::single_match_else)]
#![allow(dead_code)] // TODO: remove this

pub use token::{Token, TokenKind, Span};
pub use atom::Symbol;

mod atom;
pub mod token;
