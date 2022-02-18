/*
A palindromic number reads the same both ways. The largest palindrome made
from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

#[derive(Debug, PartialEq)]
struct Result {
    palindrome: u32,
    n_1: u32,
    n_2: u32,
}

pub fn solve() {
    let digits = 3;
    let result = largest_palindrome(digits);
    println!("The largest palindrome of the product of two {digits}-digit \
    numbers is {result}", digits=digits, result=result.palindrome);
    println!(
        "The two numbers that compose it are {} and {}",
        result.n_1,
        result.n_2,
    );
}

fn largest_palindrome(digits: u32) -> Result {
    let max_value = 10_u32.pow(digits);
    let min_value = 10_u32.pow(digits - 1);
    let mut result = Result {palindrome: 0, n_1: 0, n_2: 0};

    for n_1 in (min_value..max_value).rev() {
        // To prevent checking numbers twice, we only consider values for n_2
        // less than n_1
        for n_2 in (min_value..n_1).rev() {
            let p = n_1 * n_2;
            if is_palindrome(p) && p > result.palindrome {
                result.palindrome = p;
                result.n_1 = n_1;
                result.n_2 = n_2
            }
        }
    }
    result
}

fn is_palindrome(n: u32) -> bool {
    n == reverse(n)
}

fn reverse(n: u32) -> u32 {
    let mut result = 0;
    let mut remainder = n;

    while remainder > 0 {
        let rightmost_digit = remainder % 10;
        remainder /= 10; // remove rightmost digit
        result = result * 10 + rightmost_digit;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        for n in [1, 11, 303, 90109] {
            assert!(
                is_palindrome(n),
                "{} should be classified as a palindrome", n,
            );
        };
    }

    #[test]
    fn test_is_not_palindrome() {
        for n in [12, 110, 903, 70308] {
            assert!(
                !is_palindrome(n),
                "{} is incorrectly seen as a palindrome", n
            );
        };
    }

    #[test]
    fn check_if_example_works() {
        let expected = Result {palindrome: 9009, n_1: 99, n_2:91};
        assert_eq!(largest_palindrome(2), expected);
    }
}
