// Using code from the TRPL book with some pre-filtering
// and shortened a bit

use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut w_count = HashMap::new();

    let text = s.to_lowercase()
        .chars()
        .filter(
            |c| c.is_alphanumeric() || c.is_whitespace(),
        )
        .collect::<String>();

    for word in text.split_whitespace() {
        *w_count.entry(String::from(word)).or_insert(0) +=
            1;
    }

    w_count
}
