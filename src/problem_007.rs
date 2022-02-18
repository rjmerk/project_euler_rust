/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
that the 6th prime is 13.

What is the 10 001st prime number?
*/
pub fn solve() {
    let n = 10_001;
    let sieve_size = 104_800;
    let result = nth_prime_number(n, sieve_size);
    println!("The {n}th prime number is {result}", n=n, result=result);
}

fn nth_prime_number(n: usize, sieve_size: usize) -> usize {
    let primes = sieve_of_eratosthenes(sieve_size);
    primes[n - 1]
}

fn sieve_of_eratosthenes(size: usize) -> Vec<usize> {
    // See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Pseudocode
    let mut primes: Vec<bool> = vec! [true; size];
    primes[0] = false;
    primes[1] = false;
    let max_primes_checked: usize = (size as f64).sqrt() as usize;
    for i in 2..max_primes_checked {
        if primes[i] {
            for j in (i.pow(2)..size).step_by(i) {
                primes[j] = false;
            }
        }
    }
    let mut result: Vec<usize> = Vec::new();
    for (index, &p) in primes.iter().enumerate() {
        if p {
            result.push(index);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_example_works() {
        let n: usize = 6;
        let sieve_size: usize = 20;
        assert_eq!(nth_prime_number(n, sieve_size), 13);
    }
}
