pub fn is_armstrong_number(num: u32) -> bool {
    let p = f64::from(num).log10().ceil() as u32;

    num == Digits(num).map(|n| n.pow(p)).sum()
}

struct Digits(u32);

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => None,
            n => {
                self.0 /= 10;
                Some(n % 10)
            }
        }
    }
}
