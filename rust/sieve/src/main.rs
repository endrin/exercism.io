extern crate sieve;

use std::env;
use std::process;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    if args.len() < 2 {
        writeln!(&mut stderr, "No arguments given")
            .expect("Could not write to stderr");
        process::exit(1);
    }

    let upto = args[1].parse().unwrap();
    let primes = sieve::primes_up_to(upto);
    let amount = primes.len();

    println!(
        "Up to {} there are total of {} primes.",
        upto,
        amount
    );
}
