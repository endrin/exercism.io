// Already implemented this while solving Nth Prime.
// Taking sieve algorithm from that solution
// http://exercism.io/submissions/c1307609d2d0454abe905c43f30aa613

extern crate fnv;

mod primes;
use primes::Primes;

pub fn primes_up_to(n: usize) -> Vec<usize> {
    Primes::new().take_while(|p| p <= &n).collect()
}
