#[macro_use]
extern crate nom;
mod parser;

use nom::{IResult};

pub fn number(s: &str) -> Option<String> {
    Some(s.chars().filter(|c| c.is_digit(10)).collect())
}