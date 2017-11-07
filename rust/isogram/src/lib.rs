use std::collections::BTreeSet;
use std::iter::FromIterator;

pub fn check(s: &str) -> bool {
    let s = s.chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .collect::<String>();

    BTreeSet::from_iter(s.chars()).len() == s.len()
}
