#![feature(ascii_ctype)]

extern crate itertools;
use itertools::Itertools;

use std::ascii::AsciiExt;

pub fn encode(s: &str) -> String {
    s.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .map(swap_code)
        .chunks(5)
        .into_iter()
        .map(|cs| cs.collect::<String>())
        .join(" ")
}

pub fn decode(s: &str) -> String {
    s.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| char::to_ascii_lowercase(&c))
        .map(swap_code)
        .join("")
}

fn swap_code(c: char) -> char {
    if c.is_ascii_digit() {
        c
    } else {
        char::from(122 - (c as u8) + 97)
    }
}
