use std::env;
use std::time::{Instant, Duration};
use std::collections::HashMap;

mod problem_001;
mod problem_002;

/*
	How to call:

	to run all problems
	cargo run --release

	To run specific problem
	cargo run <int> --release
 */
fn main() {
	let problems = init_problems_map();
	let args: Vec<String> = env::args()
		.filter(|x| x != "--release")
		.collect();

	if args.len() == 1 {
		for nr in problems.keys() {
			solve_problem(*nr, &problems);
		}
	} else {
		let problem_nr = args[1].parse::<i32>();
		match problem_nr {
			Ok(nr) => solve_problem(nr - 1, &problems),
			Err(error) => println!("Could not parse number: {}", error)
		};
	}
}

fn init_problems_map() -> HashMap<i32, fn()> {
	let mut result: HashMap<i32, fn() -> ()> = HashMap::new();
	result.insert(1, problem_001::solve);
	result.insert(2, problem_002::solve);
	result
}

fn solve_problem(problem_nr: i32, problems: &HashMap<i32, fn()-> ()>) {
	println!("===========================================");
	println!("Problem {}", problem_nr);
	let now: Instant = Instant::now();
	problems[&problem_nr]();
	let duration: Duration = now.elapsed();
	println!("    Time in miliseconds: {}", duration.as_secs_f64() * 1000 as f64);
}
