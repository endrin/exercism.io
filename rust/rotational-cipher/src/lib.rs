pub fn rotate(s: &str, n: u8) -> String {
    s.chars()
        .map(|c| match c {
            'A'...'Z' => rotate_char(c, n, b'Z'),
            'a'...'z' => rotate_char(c, n, b'z'),
            _ => c,
        })
        .collect()
}

fn rotate_char(c: char, n: u8, lim: u8) -> char {
    let c = (c as u8) + n;
    (if c > lim { c - 26 } else { c }) as char
}