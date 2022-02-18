/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million
*/
use crate::problem_007::sieve_of_eratosthenes;

pub fn solve() {
    let n = 2_000_000;
    let result: usize = sieve_of_eratosthenes(n).iter().sum();
    println!("The sum of all primes below {} is {}.", n, result);
}
