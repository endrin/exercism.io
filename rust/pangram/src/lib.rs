use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let alphabet: HashSet<char> =
        "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let s_chars: HashSet<char> = s.to_lowercase()
        .chars()
        .filter(|c| match *c {
            'a'...'z' => true,
            _ => false,
        })
        .collect();
    s_chars == alphabet
}
