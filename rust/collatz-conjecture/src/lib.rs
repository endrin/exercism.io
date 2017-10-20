extern crate boolinator;
use boolinator::Boolinator;

pub fn collatz(n: u64) -> Result<u64, &'static str> {
    (n > 0)
        .ok_or("Argument must be greater than 0")
        .map(|_| CollatzSteps::new(n).count() as u64)
}

struct CollatzSteps(u64);

impl CollatzSteps {
    fn new(n: u64) -> Self {
        CollatzSteps(n)
    }
}

impl Iterator for CollatzSteps {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        (self.0 != 1).as_option().map(|_| {
            self.0 = (self.0 % 2 == 0)
                .as_some(self.0 / 2)
                .unwrap_or(self.0 * 3 + 1);
            self.0
        })
    }
}
