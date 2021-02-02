use std::{
    collections::HashSet, convert::Infallible, str::FromStr,
};

pub fn anagrams_for<'a>(
    word: &str,
    possible_anagrams: &[&'a str],
) -> HashSet<&'a str> {
    let mut checker =
        word.parse::<AnagramChecker>().unwrap();

    possible_anagrams
        .iter()
        .filter(|&&w| checker.is_anagram(w))
        .map(|&w| w)
        .collect()
}

struct AnagramChecker {
    original: Vec<char>,
    normalized: Vec<char>,
    compare_buf: Vec<char>,
}

impl FromStr for AnagramChecker {
    type Err = Infallible;

    fn from_str(word: &str) -> Result<Self, Self::Err> {
        let original = word
            .chars()
            .flat_map(|c| c.to_lowercase())
            .collect::<Vec<_>>();

        let mut normalized = original.clone();
        normalized.sort_unstable();

        let compare_buf =
            Vec::with_capacity(original.len());

        Ok(Self {
            original,
            normalized,
            compare_buf,
        })
    }
}

impl AnagramChecker {
    fn is_anagram(&mut self, word: &str) -> bool {
        self.compare_buf.clear();
        self.compare_buf.extend(
            word.chars().flat_map(|c| c.to_lowercase()),
        );

        if (self.compare_buf.len() != self.original.len())
            || (self.compare_buf == self.original)
        {
            return false;
        }

        self.compare_buf.sort_unstable();

        self.compare_buf == self.normalized
    }
}
