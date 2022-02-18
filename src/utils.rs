
pub fn factorial(n: u64) -> u64 {
    (1..n+1).fold(1, |a: u64, b: u64| a * b)
}
