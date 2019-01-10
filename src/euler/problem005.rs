//! Smallest multiple
//!
//! 2520 is the smallest number that can be divided by
//! each of the numbers from 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

use num_integer::lcm;

/// Problem 5
///
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
///
pub fn answer() -> u64 {
    //option1(20) as u64
    option3() as u64
}

/// Problem 5 (Test Sample)
///
/// What is the smallest positive number that is evenly
/// divisible by all of the numbers from 1 to 10? 2520
///
#[allow(dead_code)]
pub fn sample() -> u64 {
    option1(10) as u64
}

/// Least common multiple of 1 to n
///
/// Use a fold to solve for LCM(LCM(LCM(..., n-2), n-1), n)
#[allow(dead_code)]
fn option1(n: usize) -> usize {
    if n < 3 {
        return n;
    }
    (3..=n).fold(2, |a, x| num_integer::lcm(a, x))
}

/// Least common multiple of 1 to 20
///
/// Since the primes have no divisors, the lcm of a collection
/// of primes is the product of those primes.  Simplify the lcm
/// check by pre-multiplying the primes.
#[allow(dead_code)]
fn option2() -> usize {
    let primes = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    let others = [4_usize, 6, 8, 9, 10, 12, 14, 15, 16, 18, 19, 20];
    others.iter().fold(primes, |a, &x| lcm(a, x))
}

/// Least common multiple of 1 to 20
///
/// In addition to treating the primes separately,
/// there are a few of the other numbers I can ingnore
/// because they are unique multiples of the primes.
fn option3() -> usize {
    let primes = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    let others = [4_usize, 6, 8, 9, 10, 12, 16, 18, 19, 20];
    // I removed 14 (2*7) and 15 (3*5)
    others.iter().fold(primes, |a, &x| lcm(a, x))
}

#[allow(dead_code)]
fn option2a() -> usize {
    let primes = 2 * 3 * 5 * 7;
    let others = [4, 6, 8, 9, 10];
    others.iter().fold(primes, |a, &x| lcm(a, x))
}

#[allow(dead_code)]
fn option3a() -> usize {
    let primes = 2 * 3 * 5 * 7;
    let others = [4, 6, 8, 9];
    // I don't need to check 10 since it is a multiple of 2 and 5
    // Nothing else is a multiple of the remaining numbers (3,7)
    others.iter().fold(primes, |a, &x| lcm(a, x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sample() {
        assert_eq!(option1(10), 2520);
    }
    #[test]
    pub fn test_sample_opt2() {
        assert_eq!(option2a(), 2520);
    }
    #[test]
    pub fn test_sample_opt3() {
        assert_eq!(option3a(), 2520);
    }
}
