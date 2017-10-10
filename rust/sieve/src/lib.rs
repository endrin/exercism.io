// Already implemented this while solving Nth Prime.
// Taking sieve algorithm from that solution
// http://exercism.io/submissions/c1307609d2d0454abe905c43f30aa613

extern crate fnv;

mod primes;
use primes::Primes;

pub fn primes_up_to(n: usize) -> Vec<usize> {
    let mut primes = if n < 1000 {
        Vec::new()
    } else {
        Vec::with_capacity(estimated(n))
    };
    primes.extend(Primes::new().take_while(|p| p <= &n));
    primes
}

fn estimated(n: usize) -> usize {
    let x = n as f64;
    (x / (x.ln() - 1f64)).ceil() as usize
}
