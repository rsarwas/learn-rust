//! Prints and times solutions to Project Euler problems
//!
//! With no arguments, it runs the function run_test, which can be set up
//! for testing a solution to a particular problem.  With a single numerical
//! argument, it will time the solution to that particular problem, if
//! available.  With *all* it will solve all available problems.
mod euler;

/// Test Runner
///
/// Edit the body of this function to call code during development
fn run_test() {
    println!("{}", euler::problem004::sample());
    //euler::math::fibonacci::benchmark();
}

/// Run only one selected problem.
///
/// The input (n) must be one of the solved problems.
/// This is useful for timing tests
fn run_one(n: u32) {
    if let Some(function) = FUNCTIONS.iter().find(|&&t| t.0 == n) {
        let function = function.1;
        let start = std::time::Instant::now();
        let answer = function();
        let duration = start.elapsed();
        println!("Euler {} = {}; found in {:?}", n, answer, duration);
    } else {
        println!("Euler {} not available.", n);
    }
}

/// Run all of the solved Euler problems.
///
/// See the FUNCTIONS constant for a list of the
/// solved problems.
fn run_all() {
    for (n, euler) in FUNCTIONS.iter() {
        let start = std::time::Instant::now();
        let answer = euler();
        let duration = start.elapsed();
        println!("Euler {} = {}; found in {:?}", n, answer, duration);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let arg = &args[1];
        match arg.parse() {
            Ok(n) => run_one(n),
            Err(_) => run_all(),
        }
    } else {
        run_test();
    }
}

/// List of solved Euler Problems
///
/// The first item in the tuple is the problem number, and the
/// second is the function that yields the solution as a u64.
const FUNCTIONS: [(u32, fn() -> u64); 5] = [
    (1, || euler::problem001::answer()),
    (2, || euler::problem002::answer()),
    (3, || euler::problem003::answer()),
    (4, || euler::problem004::answer()),
    (5, || euler::problem005::answer()),
    // Add new solutions here
    // Update the length in the constant type when done
];
