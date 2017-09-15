pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .flat_map(|word| word.chars().take(1))
        .flat_map(char::to_uppercase)
        .collect()
}
