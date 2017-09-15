mod primes;
use primes::Primes;

pub fn factors(n: usize) -> Vec<usize> {
    Factors::find_for(n).collect()
}

struct Factors {
    multiple: usize,
}

impl Factors {
    fn find_for(multiple: usize) -> Factors {
        Factors { multiple }
    }
}

impl Iterator for Factors {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.multiple == 1 {
            None
        } else {
            let next_factor = Primes::new()
                .find(|p| self.multiple % p == 0)
                .unwrap();
            self.multiple /= next_factor;
            Some(next_factor)
        }
    }
}