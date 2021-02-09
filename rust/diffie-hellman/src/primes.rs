use rand::{
    distributions::Uniform, seq::IteratorRandom, Rng,
    SeedableRng,
};
use rand_chacha::ChaCha20Rng;

use crate::fastexp::exp_mod;

// List of all primes before 256 except 2.
// Grabbed from: https://www.factors-of.com/
static SMALL_PRIMES: [u64; 53] = [
    3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,
    53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107,
    109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
    173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229,
    233, 239, 241, 251,
];

pub struct PrimeGen {
    rng: ChaCha20Rng,
}

impl PrimeGen {
    pub fn new() -> Self {
        Self {
            rng: ChaCha20Rng::from_entropy(),
        }
    }

    pub fn random_below(&mut self, n: u64) -> Option<u64> {
        match n {
            0 | 1 | 2 => None,
            3 => Some(2),
            4..=256 => self.small_below(n),
            _ => self.large_below(n),
        }
    }

    fn small_below(&mut self, n: u64) -> Option<u64> {
        SMALL_PRIMES
            .iter()
            .copied()
            .take_while(|p| p < &n)
            .choose(&mut self.rng)
    }

    fn large_below(&mut self, n: u64) -> Option<u64> {
        loop {
            let candidate =
                self.rng.gen_range(2..n - 1) | 1;
            if self.is_prime(candidate) {
                break Some(candidate);
            }
        }
    }

    fn is_prime(&mut self, p: u64) -> bool {
        if p < 256 {
            return SMALL_PRIMES.contains(&p);
        }

        if divisible_by_small(p) {
            return false;
        }

        let test = RabinMillerTest::with_candidate(p);
        (&mut self.rng)
            .sample_iter(Uniform::new(2, p))
            .take(5)
            .all(|w| test.passed_versus(w))
    }
}

fn divisible_by_small(p: u64) -> bool {
    let upper_bound = (p as f64).sqrt().floor() as u64;
    SMALL_PRIMES
        .iter()
        .take_while(|&n| n < &upper_bound)
        .any(|&n| p % n == 0)
}

#[derive(Clone, Copy)]
struct RabinMillerTest {
    p: u64,
    b: u32,
    m: u64,
}

impl RabinMillerTest {
    fn with_candidate(p: u64) -> Self {
        // Number of times 2 divides p - 1
        let b = ((p - 1) & !(p - 2)).trailing_zeros();
        // m is chosen so p = 1 + 2^b * m
        let m = (p - 1) >> b;

        Self { p, b, m }
    }

    fn passed_versus(&self, w: u64) -> bool {
        let mut z = exp_mod(w, self.m, self.p);

        if z == 1 || z == self.p - 1 {
            return true;
        }

        for _j in 1..=self.b {
            z = exp_mod(z, 2, self.p);

            if z == 1 {
                return false;
            }

            if z == self.p - 1 {
                return true;
            }
        }

        // if j == b and z == p - 1 still not reached,
        // candidate is not prime
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_primes() {
        let mut pg = PrimeGen::new();

        assert!(pg.is_prime(131));
        assert!(pg.is_prime(137));
        assert!(pg.is_prime(227));
    }

    #[test]
    fn small_not_primes() {
        let mut pg = PrimeGen::new();

        assert!(!pg.is_prime(93));
        assert!(!pg.is_prime(133));
        assert!(!pg.is_prime(249));
    }

    #[test]
    fn large_primes() {
        let mut pg = PrimeGen::new();

        assert!(pg.is_prime(751));
        assert!(pg.is_prime(1237));
        assert!(pg.is_prime(1999));
    }

    #[test]
    fn large_not_primes() {
        let mut pg = PrimeGen::new();

        assert!(!pg.is_prime(5687));
        assert!(!pg.is_prime(7671));
        assert!(!pg.is_prime(9757));
    }
}
