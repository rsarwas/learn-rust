//! Fibonacci Numbers
//!
//! Each new term in the Fibonacci sequence is generated by adding the previous two terms.
//! By starting with 1 and 2, the first 10 terms will be:
//!
//!    1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//!
//! The common definitions (per wikipedia) start with 0, 1:
//!
//!   0, 1, 1, 2, 3, 5, 8, ...

/// Returns the nth Fibonacci number, starting at 0
///
/// Typical recursive solution. Very inefficient due to redundant calculations.
/// For example, fib(5) calculates fib(3) and fib(4), fib(4) then recalculates fib(3)
///
/// # Examples
/// ```
/// assert_eq!(fib(0), 0);
/// assert_eq!(fib(1), 1);
/// assert_eq!(fib(6), 8);
/// ```
pub fn fib(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

/// Returns the nth Fibonacci number, starting at 0
///
/// A loop based solution which counts upto the requested number.
/// This is a more efficient solution than the recursive method.
///
/// # Examples
/// ```
/// assert_eq!(fibonacci(0), 0);
/// assert_eq!(fibonacci(1), 1);
/// assert_eq!(fibonacci(6), 8);
/// ```
pub fn fibonacci(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut x = (0, 1);
    for _ in 1..n {
        x = (x.1, x.0 + x.1)
    }
    x.1
}

/// Returns a vector with the first n Fibonacci numbers, starting at 0
///
/// Similar in performance to the loop based solution.
/// Uses more memory to memoize the sequence.
/// Vector allocation is efficient, because we know up front
/// how many elements there will be.
///
/// # Examples
/// ```
/// let fibs = fibonacci_first(10);
/// assert_eq!(fibs(0), 0);
/// assert_eq!(fibs(1), 1);
/// assert_eq!(fibs(6), 8);
/// ```
pub fn fibonacci_first(n: usize) -> Vec<usize> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        if i < 2 {
            v.push(i);
        } else {
            v.push(v[i - 1] + v[i - 2]);
        }
    }
    v
}

/// Returns a vector of all Fibonacci numbers upto n, starting at 0
///
/// Similar in performance to the loop based solution.
/// Uses more memory to memoize the sequence.
/// Vector allocation is less efficient, because the number of
/// elements is unknown until we get to the requeste element.
///
/// # Examples
/// ```
/// let fibs = fibonacci_upto(100);
/// assert_eq!(fibs(0), 0);
/// assert_eq!(fibs(1), 1);
/// assert_eq!(fibs(10), 55);
/// ```
pub fn fibonacci_upto(n: usize) -> Vec<usize> {
    let mut v = Vec::new();
    v.push(0);
    if n == 0 {
        return v;
    };
    v.push(1);
    if n == 1 {
        return v;
    };
    for i in 2.. {
        let x = v[i - 1] + v[i - 2];
        if x > n {
            break;
        };
        v.push(x);
    }
    v
}

/// State for a fibonacci iterator
///
/// Has no public state nor methods, except
/// for next() as part of the Iterator trait.
pub struct Fib {
    x: (usize, usize),
}

impl Fib {
    fn new() -> Fib {
        Fib { x: (0, 1) }
    }
}

impl Iterator for Fib {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let next = self.x.0 + self.x.1;
        self.x = (self.x.1, next);
        Some(next)
    }
}

/// Returns an iterator that produces an infinite sequence of Fibonacci numbers
///
/// The sequence skips 0, 1 and starts with 1, 2, 3, 5, ...
/// This solution is fast and memory efficient, and could be used
/// as the basis for all other funtions in this module.
pub fn iter() -> Fib {
    Fib::new()
}

/// Returns an iterator that produces an infinite sequence of Fibonacci numbers
///
/// The sequence skips 0 and starts with 1, 1, 2, 3, 5, ...
/// This solution is the fastest and most memory efficient,
/// but the [scan] function is a little hard to read.
///
/// Idea came from this [thread] on the rust forums.
///
/// [scan]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan
/// [thread]: https://users.rust-lang.org/t/tuples-destructuring-assignment/11028/2
pub fn iter2() -> impl Iterator<Item = usize> {
    (1..).scan((0, 1), |x, _| {
        *x = (x.1, x.0 + x.1);
        Some(x.0)
    })
}

// TODO: Create a generator based solution.
// This will not require a struct to hold state
// Generators are not in the stable release (as of 2018-01-07)

/// Benchmark the various fibonacci functions
///
/// The recursive solution is horrible.
/// The loop solution is very fast if you only need one number.
/// The iterator is just as fast at about < 20ns per number.
/// The vector based solutions are also fast, but only useful if you need
/// to scan the iterator sequence multiple times.
#[allow(dead_code)]
pub fn benchmark() {
    println!("Recursive solution for specific fibonacci number");
    for i in 1..20 {
        let start = std::time::Instant::now();
        println!("fib #{} = {} in {:?}", i, fib(i), start.elapsed());
    }
    println!("Loop solution for specific fibonacci number");
    for i in 1..20 {
        let start = std::time::Instant::now();
        println!("fib #{} = {} in {:?}", i, fibonacci(i), start.elapsed());
    }
    println!("Create a vector of the first 20 fibonacci numbers");
    let start = std::time::Instant::now();
    let fibs = fibonacci_first(20);
    println!(" created in {:?}", start.elapsed());
    for (i, f) in fibs.iter().enumerate() {
        println!("fib #{} = {}", i, f);
    }
    println!("Create a vector of the fibonacci numbers upto 10,000");
    let start = std::time::Instant::now();
    let fibs = fibonacci_upto(10_000);
    println!(" created in {:?}", start.elapsed());
    for (i, f) in fibs.iter().enumerate() {
        println!("fib #{} = {}", i, f);
    }
    println!("Iterator solution");
    let mut start = std::time::Instant::now();
    for (i, f) in iter().take(20).enumerate() {
        println!("fib #{} = {} in {:?}", i, f, start.elapsed());
        start = std::time::Instant::now();
    }
    println!("Iterator solution #2");
    let mut start = std::time::Instant::now();
    for (i, f) in iter2().take(20).enumerate() {
        println!("fib #{} = {} in {:?}", i, f, start.elapsed());
        start = std::time::Instant::now();
    }
}
