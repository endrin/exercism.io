use std::collections::BTreeSet;

pub fn check(s: &str) -> bool {
    let s = s.chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .collect::<String>();

    s.chars().collect::<BTreeSet<_>>().len() == s.len()
}
