/*
The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
It can be seen that this sequence (starting at 13 and finishing at 1) contains
 10 terms. Although it has not been proved yet (Collatz Problem), it is
thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
 */

// answer should be 837799 length 524

use std::collections::HashMap;

pub fn solve()
{
    let x: u64 = 137846528820;
    println!("{}", x);
    let mut longest_chain: u64 = 0;
    let mut length_longest_chain: u64 = 0;
    let mut chain_length_dict: HashMap<u64, u64> = HashMap::new();

    for starting_number in 1..1_000_000 {
        let chain_length = length_of_chain(starting_number, &mut chain_length_dict);
        if chain_length > length_longest_chain {
            longest_chain = starting_number;
            length_longest_chain = chain_length;
        }
    }
    println!(
        "The starting number under 1 million with longest Collatz chain is= {}, with length {}",
        longest_chain,
        length_longest_chain,
    );
}

fn length_of_chain(
    starting_number: u64,
    chain_length_dict: &mut HashMap<u64, u64>
) -> u64
{
    let mut n = starting_number;
    let mut chain_length = 1;
    while n > 1 {
        let v = chain_length_dict.get(&n);
        if let Some(stored_chain_length) = v {
            chain_length += stored_chain_length;
            chain_length_dict.insert(starting_number, chain_length);
            return chain_length;
        }
        n = match n % 2 {
            0 => n / 2,
            _ => n * 3 + 1,
        };
        chain_length += 1;
    }
    chain_length_dict.insert(starting_number, chain_length);
    chain_length
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn check_if_example_works()
    {
        assert_eq!(length_of_chain(13, &mut HashMap::new()), 10);
    }
}
