/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we
get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/
const LIMIT: u64 = 1_000;

pub fn solve() {
    let result: u64 = sum_multiples(3) + sum_multiples(5) - sum_multiples(15);
    println!("The answer for n = {} is {}", LIMIT, result);
}

fn sum_multiples(step: usize) -> u64 {
    (0..LIMIT).step_by(step).sum()
}
