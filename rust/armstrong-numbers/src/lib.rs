pub fn is_armstrong_number(num: u32) -> bool {
    let p = Digits(num).count() as u32;
    let checksum =
        Digits(num).map(|n| n.pow(p)).sum::<u32>();

    checksum == num
}

struct Digits(u32);

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let d = self.0 % 10;
            self.0 /= 10;
            Some(d)
        }
    }
}
