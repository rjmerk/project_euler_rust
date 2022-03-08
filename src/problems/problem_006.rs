/*
The sum of the squares of the first ten natural numbers is,
    1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,
    (1 + 2 + ... + 10)^2 = 552 = 3025

Hence the difference between the sum of the squares of the first ten natural
numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred
natural numbers and the square of the sum.
*/
pub fn solve() {
    let n = 100;
    let result = square_of_sum(n) - sum_of_squares(n);
    println!(
    "The difference between the sum of squares and the square of the sum for the first {n} natural numbers is {result}",
        n=n,
        result=result,
    )
}

fn square_of_sum(n: u64) -> u64 {
    let s: u64 = (1..n+1).sum();
    s.pow(2)
}

fn sum_of_squares(n: u64) -> u64 {
    let squares = (1..n+1).map(|x: u64| x.pow(2));
    squares.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_of_sum() {
        assert_eq!(square_of_sum(10), 3025);
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(10), 385);
    }

    #[test]
    fn check_if_example_works() {
        let n = 10;
        let actual = square_of_sum(n) - sum_of_squares(n);
        assert_eq!(actual, 2640);

    }
}
