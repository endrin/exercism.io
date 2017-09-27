// Tried to implement infinite primes sequence
// using Iterator and the Sieve of Eratosthenes

use std::collections::BTreeMap;

pub struct Primes {
    current: usize,
    not_primes: BTreeMap<usize, usize>,
}

impl Primes {
    pub fn new() -> Primes {
        Primes {
            current: 2,
            not_primes: BTreeMap::new(),
        }
    }
}

impl Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for q in self.current.. {
            if let Some(p) = self.not_primes.remove(&q) {
                let mut x = q.checked_add(p);
                while x.is_some() &&
                    self.not_primes
                        .contains_key(&x.unwrap())
                {
                    x = x.and_then(|x_| x_.checked_add(p));
                }
                x.map(|x_| self.not_primes.insert(x_, p));
            } else {
                q.checked_mul(q).map(|q_squared| {
                    self.not_primes.insert(q_squared, q)
                });
                self.current = q + 1;
                return Some(q);
            }
        }
        unreachable!();
    }
}
