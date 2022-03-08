/*The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

pub fn solve() {
    let n = 600_851_475_143;
    let prime_factors = prime_factors(n);
    let largest = prime_factors.iter().max().unwrap();
    println!("The largest prime factor of {} is {}.", n, largest);
    println!("The prime factors are {:?}.", prime_factors);
}

fn prime_factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut n: u64 = n;
    let mut factor: u64 = 3;
    while n > 1 {
        while n % factor == 0 {
            n /= factor;
            result.push(factor);
        }
        factor += 2
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_example_works() {
        let actual: Vec<u64> = prime_factors(13195);
        assert_eq!(actual, vec![5, 7, 13, 29]);
    }
}
