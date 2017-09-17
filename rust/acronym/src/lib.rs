use std::iter;

pub fn abbreviate(phrase: &str) -> String {
    // Trim extra spaces from the phrase
    let phrase = phrase
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let phrase_bytes = phrase.as_bytes();

    // First character is always included in acronym
    let first = iter::once(phrase_bytes[0] as char);

    first
        .chain((0..phrase.len() - 1).filter_map(|idx| {
            let current = phrase_bytes[idx] as char;
            let next = phrase_bytes[idx + 1] as char;
            if current.is_whitespace() || current == '-' ||
                (current.is_lowercase() &&
                    next.is_uppercase())
            {
                Some(next)
            } else {
                None
            }
        }))
        .flat_map(char::to_uppercase)
        .collect()
}
