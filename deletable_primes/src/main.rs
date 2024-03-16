pub mod library;
use std::env;

use crate::library::is_prime::{count_single_digit_numbers, deletable_primes, is_prime};

fn main() {
    for argument in env::args().skip(1) {
        println!(
            "{} is prime ? -> {}",
            argument,
            is_prime(argument.parse().unwrap())
        );
        let deletable_primes = deletable_primes(argument.parse().unwrap());
        println!(
            "Deletable primes: {:?}",
            count_single_digit_numbers(deletable_primes)
        );
    }
}
