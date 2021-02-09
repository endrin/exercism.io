pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        std::iter::repeat_with(String::new)
            .take(digits.len() + 1)
            .collect()
    } else {
        digits
            .as_bytes()
            .windows(len)
            .map(|s| String::from_utf8_lossy(s).into())
            .collect()
    }
}
