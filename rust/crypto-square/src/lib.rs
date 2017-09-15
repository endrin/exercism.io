// Continuing to explore itertools crate

extern crate itertools;
use itertools::Itertools;

use std::ascii::AsciiExt;

pub fn encrypt(input: &str) -> String {
    let input = normalized(input);
    let c = (input.len() as f64).sqrt().ceil() as usize;
    (0..c)
        .into_iter()
        .map(|offset| {
            input
                .chars()
                .skip(offset)
                .step(c)
                .collect::<String>()
        })
        .join(" ")
}

fn normalized(s: &str) -> String {
    s.chars()
        .filter_map(
            |c| if c.is_ascii() && c.is_alphabetic() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            },
        )
        .collect()
}