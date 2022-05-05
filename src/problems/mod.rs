mod problem_001;
mod problem_002;
mod problem_003;
mod problem_004;
mod problem_005;
mod problem_006;
mod problem_007;
mod problem_008;
mod problem_009;
mod problem_010;
mod problem_011;
mod problem_012;
mod problem_013;
mod problem_014;

use std::collections::HashMap;

pub fn init_problems_map() -> HashMap<i32, fn()> {
    let mut result: HashMap<i32, fn() -> ()> = HashMap::new();
    result.insert(1, problem_001::solve);
    result.insert(2, problem_002::solve);
    result.insert(3, problem_003::solve);
    result.insert(4, problem_004::solve);
    result.insert(5, problem_005::solve);
    result.insert(6, problem_006::solve);
    result.insert(7, problem_007::solve);
    result.insert(8, problem_008::solve);
    result.insert(9, problem_009::solve);
    result.insert(10, problem_010::solve);
    result.insert(11, problem_011::solve);
    result.insert(12, problem_012::solve);
    result.insert(13, problem_013::solve);
    result.insert(14, problem_014::solve);

    result
}
