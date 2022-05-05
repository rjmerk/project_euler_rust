/*
Starting in the top left corner of a 2×2 grid, and only being able to move to
the right and down, there are exactly 6 routes to the bottom right corner.

How many such routes are there through a 20×20 grid?
*/

pub fn solve()
{
    let size = 20;
    let result = nr_routes_in_grid(size);
    println!("the number of routes through a {size}x{size} grid = {}", result)
}


fn nr_routes_in_grid(size: usize) -> u64
{
    // We use a flat vector instead of a 2-dimensional nested vector
    // index() is used to easily calculate the index of that flat vector
    // for a given x,y position.
    let index = |y, x| y * size + x;
    // the grid needs to be one size larger, because we look at intersections,
    // not the grid centers.
    // So for example a 2x2 grid has 3 intersections on each size
    let mut intersection = vec![0; (size + 1) * (size + 1)];

    // the right-most column and the bottom row have route lengths 1
    for n in 0..size {
        intersection[index(size, n)] = 1;
        intersection[index(n, size)] = 1;
    }
    // We now iterate over all the inner values, starting at the bottom right
    // The nr. of routes in a intersection is the nr. of routes
    // to the right plus the nr. of routes to the bottom.
    for x in (0..size).rev() {
        for y in (0..size).rev() {
            intersection[index(y, x)] =
                intersection[index(y + 1, x)]
                + intersection[index(y, x + 1)];
        }
    }
    intersection[0]
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn check_if_example_works()
    {
        assert_eq!(nr_routes_in_grid(2), 6);
        assert_eq!(nr_routes_in_grid(20), 137846528820);
    }
}
