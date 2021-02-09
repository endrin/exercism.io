mod fastexp;
mod primes;

use fastexp::exp_mod;
use primes::PrimeGen;

pub fn private_key(p: u64) -> u64 {
    PrimeGen::new()
        .random_below(p)
        .expect("argument must be greater than 2")
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    exp_mod(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    exp_mod(b_pub, a, p)
}
