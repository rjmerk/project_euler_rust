/*
2520 is the smallest number that can be divided by each of the numbers from 1
to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the
 numbers from 1 to 20?
*/
use super::utils::factorial;

pub fn solve() {
    let n = 20;
    let result = smallest_evenly_divisible_number(n);
    println!(
        "{result} is the smallest positive number divisible by all numbers from 1 to {n}",
        result=result,
        n=n,
    )
}

fn smallest_evenly_divisible_number(n: u64) -> u64 {
    let max: u64 = factorial(n) + 1;
    for x in (n..max).step_by(n as usize) {
        if is_evenly_divisible(x, n) { return x}
    }
    0
}

fn is_evenly_divisible(x: u64, max_factor: u64) -> bool {
    for factor in 2..max_factor {
        if !(x % factor == 0) { return false }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_example_works() {
        assert_eq!(smallest_evenly_divisible_number(10), 2520);
    }
}
