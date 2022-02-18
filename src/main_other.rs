// pub struct Collatz {
//     n: u64,
// }
//
// impl Collatz {
//     pub fn new(n: u64) -> Collatz {
//         Collatz { n }
//     }
// }
//
// impl Iterator for Collatz {
//     type Item = u64;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.n % 2 == 0 {
//             self.n = self.n / 2;
//             return Some(self.n);
//         }
//         if self.n == 1 {
//             return None;
//         }
//         self.n = self.n * 3 + 1;
//         Some(self.n)
//     }
// }
//
// pub fn count_collatz(n: u32, lengths: &mut [u16; 500_000]) -> u16 {
//     if n < 500_000 && lengths[n as usize] > 0 {
//         lengths[n as usize]
//     } else {
//         if n == 1 {
//             return 1;
//         } else {
//             let val = 1 +
//                 match n % 2 {
//                     0 => count_collatz(n / 2, lengths),
//                     _ => count_collatz(n * 3 + 1, lengths),
//                 };
//             if n < 500_000 {
//                 lengths[n as usize] = val;
//             }
//             val
//         }
//     }
// }
//
// pub fn longest_collatz_memo(highest: u32) -> u32 {
//     let mut max_length = 0;
//     let mut lengths: [u16; 500_000] = [0; 500_000];
//     let mut cause = 0;
//     for i in 1..highest + 1 {
//         let length = count_collatz(i, &mut lengths);
//         if length > max_length {
//             cause = i;
//             max_length = length;
//         }
//     }
//     cause
// }
//
// pub fn longest_collatz(highest: usize) -> usize {
//     let mut max_length = 0;
//     let mut cause = 0;
//     for i in 1..highest + 1 {
//         let mut length = 1;
//         let mut n = i;
//         loop {
//             if n == 1 {
//                 break;
//             }
//             n = match n % 2 {
//                 0 => n / 2,
//                 _ => n * 3 + 1,
//             };
//             length += 1;
//         }
//         if length > max_length {
//             max_length = length;
//             cause = i;
//         }
//     }
//     cause
// }
// use std::time::{Instant};
//
// fn main() {
//     let now = Instant::now();
//     let x = longest_collatz(1000_000);
//     println!("Time in miliseconds: {}", now.elapsed().as_millis());
//     println!("Longest is n = {}, with length", x);
// }
