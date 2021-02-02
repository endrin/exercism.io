#![feature(bool_to_option)]

use std::usize;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: &[u8] = isbn.as_ref();
    if isbn.is_empty() {
        return false;
    }

    let valid_digits = isbn[..isbn.len() - 1]
        .iter()
        .filter_map(|&b| {
            matches!(b, b'0'..=b'9')
                .then(|| digit_to_num(b))
                .flatten()
        })
        .collect::<Vec<_>>();

    if valid_digits.len() < 9 {
        return false;
    }

    let sum = valid_digits
        .iter()
        .enumerate()
        .map(|(pos, digit)| (10 - pos) * digit)
        .sum::<usize>();

    digit_to_num(isbn[isbn.len() - 1])
        .map(|check| (check + sum) % 11 == 0)
        .unwrap_or(false)
}

fn digit_to_num(digit: u8) -> Option<usize> {
    match digit {
        b'0'..=b'9' => Some(usize::from(digit - b'0')),
        b'X' => Some(10),
        _ => None,
    }
}
