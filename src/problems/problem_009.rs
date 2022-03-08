/*
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a2^ + b^2 = c^2
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
 */
pub fn solve() {
    let (a, b, c) = find_pythagorean_triplet_summing_up_to(1000);
    let product = a * b * c;
    println!("{}", 25/2);

    println!("The Pythagorean triplet for which a + b + c = 1000 is {}, {}, {}", a, b, c);
    println!("The product for which is {}", product);
}

fn find_pythagorean_triplet_summing_up_to(n: u32) -> (u32, u32, u32) {
    /*
    Once we know a and b, we can calculate c as the three need to add up to n.

    Furthermore, a or b cannot be larger than n/2, because this would mean
    that c can be at most half n minus 1. So if a (or b) is higher than n/2,
    then c^2 is already lower than a^2 alone, let alone a^2 + b^2.
     */
    for a in 1..n/2 {
        for b in 1..n/2 {
            let c = n - a - b;
            if is_pythagorean_triplet(a, b, c) {
                return (a, b, c);
            }
        }
    }
    // this shouldn't happen
    (0, 0, 0)
}

fn is_pythagorean_triplet(a: u32, b: u32, c: u32) -> bool {
    a*a + b*b == c*c
}
