// Reusing Primes iterator from my Nth Prime solution

use std::collections::HashMap;

pub struct Primes {
    current: usize,
    not_primes: HashMap<usize, usize>,
}

impl Primes {
    pub fn new() -> Primes {
        Primes {
            current: 2,
            not_primes: HashMap::new(),
        }
    }
}

impl Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for q in self.current.. {
            if let Some(p) = self.not_primes.remove(&q) {
                let mut x = q + p;
                while self.not_primes.contains_key(&x) {
                    x += p;
                }
                self.not_primes.insert(x, p);
            } else {
                q.checked_mul(q).and_then(|q_squared| {
                    self.not_primes.insert(q_squared, q)
                });
                self.current = q + 1;
                return Some(q);
            }
        }
        unreachable!();
    }
}