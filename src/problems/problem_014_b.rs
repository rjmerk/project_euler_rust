
pub fn longest_collatz(highest: usize) -> usize {
    let mut max_length = 0;
    let mut result = 0;
    for i in 1..highest + 1 {
        let mut length = 1;
        let mut n = i;
        while n > 1 {
            n = match n % 2 {
                0 => n / 2,
                _ => n * 3 + 1,
            };
            length += 1;
        }
        if length > max_length {
            max_length = length;
            result = i;
        }
    }
    result
}


pub fn solve() {
    let result = longest_collatz(1_000_000);
    println!("Longest is n = {}, with length", result);
}
