pub fn check(s: &str) -> bool {
    let mut s_chars = s.chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .collect::<Vec<_>>();
    s_chars.sort();

    let total_chars = s_chars.len();
    s_chars.dedup();
    let unique_chars = s_chars.len();

    unique_chars == total_chars
}
