/// Prints and times solutions to Project Euler problems
///
/// With no arguments, it runs the function run_test, which can be set up
/// for testing a solution to a particular problem.  With a single numerical
/// argument, it will time the solution to that particular problem, if
/// available.  With the *all* it will solve all available problems.
///
mod euler;

// Edit the body of this function to call code during development
fn run_test() {
    //euler::math::fibonacci::benchmark();
    //euler::problem002::test();
    euler::problem002::sample();
}

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
            Err(_) => run_all()
        }
    } else {
        run_test();
    }
}

// Add new solutions here
// The first item in the tuple is the problem number, and the
// second is the funtion that yields the solution.
// be sure to update the length in the constant type when adding
// to the array
const FUNCTIONS :[(u32, fn() -> u64); 2] = [
    (1, || euler::problem001::answer()),
    (2, || euler::problem002::answer())
];
