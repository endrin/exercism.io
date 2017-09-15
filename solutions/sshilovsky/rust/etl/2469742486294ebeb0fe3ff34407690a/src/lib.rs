use std::ascii::AsciiExt;
use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    input.iter()
        .flat_map(
            |(&score, chars)| {
                chars.iter()
                    .map(move |ch| (ch.to_ascii_lowercase(), score))
            }
        )
        .collect()
}