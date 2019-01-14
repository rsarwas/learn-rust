//! Consecutive positive divisors
//!
//! Find the number of integers 1 < n < 10^7, for which n and n + 1 have the
//! same number of positive divisors.
//!
//! For example, 14 has the positive divisors 1, 2, 7, 14 while 15 has 1, 3, 5, 15.

use super::math::isqrt;

/// Problem 179
///
/// Find the number of integers 1 < n < 10^7, for which n and n + 1 have the
/// same number of positive divisors.
pub fn answer() -> u64 {
    option1(10_000_000) as u64
}

/// Problem 179 (Test Sample)
///
/// Find the number of integers 1 < n < 25, for which n and n + 1 have the
/// same number of positive divisors.
#[allow(dead_code)]
pub fn sample() {
    print_divisor_counts(25);
    println!("{} pairs below 25", option1(25));
}

pub fn print_divisor_counts(n: usize) {
    let sieve = primal::Sieve::new(isqrt(n));
    for i in 2..=(n + 1) {
        println!("{} has {} divisors", i, divisor_count(i, &sieve));
    }
}

/// Returns the count of consecutive numbers with the same divisor count
///
/// Uses the prime sieve from the primal crate
pub fn option1(n: usize) -> usize {
    let sieve = primal::Sieve::new(n);
    let mut count = 0;
    let mut prev = 0;
    for i in 2..=(n + 1) {
        let num = divisor_count(i, &sieve);
        if num == prev {
            count += 1;
        }
        prev = num;
    }
    count
}

/// Count the divisors of n
///
/// The count includes 1 and the n
/// This function uses the fact the number of divisors is
///     (c1+1)*(c2+1)* ... * (ck+1)
/// where n = p1^c1 * p2^c2 * ... * pk^ck
/// where pi are the prime factors, and ci is the number of times that
/// prime factor occurs
///
/// Since we will call this many times, we have the caller generate a
/// sieve of prime numbers that can be used to find the divisors
pub fn divisor_count(n: usize, sieve: &primal::Sieve) -> usize {
    if sieve.is_prime(n) {
        return 2;
    }
    let mut total = 1;
    // factor() returns a Result object; I'll panic if it is an Err()
    // It should only error if I don't give it enough primes, so it should be ok.
    // The result is a vector of (prime#, count) tuples.
    // I could write my own prime factor finders/counter, but I will need a list
    // of prime numbers to do it, so it won't be any faster than this solution.
    let factors = sieve.factor(n).unwrap();
    for (_, c) in factors.iter() {
        total *= c + 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn divisor_count_test1() {
        assert_eq!(option1(25), 3);
    }
}
