//! Largest prime factor
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//! What is the largest prime factor of the number 600851475143 ?

use super::math;

/// Euler Problem # 3
///
/// What is the largest prime factor of the number 600851475143 ?
pub fn answer() -> u64 {
    // This takes about 1.3ms with a release build
    option2(600_851_475_143) as u64
}

/// Euler Problem # 3 (Test Sample)
///
/// What is the largest prime factor of the number 13195 ?
#[allow(dead_code)]
pub fn sample() {
    println!("  Option #1 = {}", option1(13195));
    println!("  Option #2 = {}", option2(13195));
}

/// What is the largest prime factor of n
///
/// Option 1, search the list of primes from
/// 2 .. sqrt(n) in reverse order until one
/// is divisible.  If none is divisible, return n (prime)
///
///
/// # Examples
/// ```
/// assert_eq!(option1(6), 3)
/// assert_eq!(option1(11), 11)
/// assert_eq!(option1(27), 3)
/// assert_eq!(option1(13195), 29)
/// ```
pub fn option1(n: usize) -> usize {
    // Unfortunately, primal::Sieve::new().primes_from()
    // is a single ended iterator and cannot be reversed
    // we need to collect it as a vector to reverse it.
    let primes: Vec<usize> = primal::Sieve::new(math::isqrt(n)).primes_from(2).collect();
    for p in primes.iter().rev() {
        if n % *p == 0 {
            return *p;
        }
    }
    n
}

/// What is the largest prime factor of n
///
/// Option 2, use the list of primes from
/// 2 .. sqrt(n) in order and if the number is
/// divisible by that factor, then divide as many times
/// as possible before trying the next prime number.
/// return what ever is left when we run out of primes, or
/// we are the same size or smaller than the current prime.
///
/// Surprisingly, this is faster than the simpler solution above.
/// I'm guessing it is because we avoid the monkey
/// business with creating and reversing a vector.
///
/// # Examples
/// ```
/// assert_eq!(option2(6), 3)
/// assert_eq!(option2(11), 11)
/// assert_eq!(option2(27), 3)
/// assert_eq!(option2(13195), 29)
/// ```
pub fn option2(n: usize) -> usize {
    let sieve = primal::Sieve::new(math::isqrt(n));
    let mut factor = n;
    for p in sieve.primes_from(2) {
        let mut quot = factor / p;
        let mut rem = factor % p;
        while rem == 0 && factor > p {
            factor = quot;
            if factor <= p {
                break;
            } // I think I can prove factor = p is a limit
            quot = factor / p;
            rem = factor % p;
        }
    }
    factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn gpf_of_6() {
        assert_eq!(option1(6), 3)
    }
    #[test]
    pub fn gpf_of_11() {
        assert_eq!(option1(11), 11)
    }
    #[test]
    pub fn gpf_of_27() {
        assert_eq!(option1(27), 3)
    }
    #[test]
    pub fn gpf_of_13195() {
        assert_eq!(option1(13195), 29)
    }
    #[test]
    pub fn gpf_of_6a() {
        assert_eq!(option2(6), 3)
    }
    #[test]
    pub fn gpf_of_11a() {
        assert_eq!(option2(11), 11)
    }
    #[test]
    pub fn gpf_of_27a() {
        assert_eq!(option2(27), 3)
    }
    #[test]
    pub fn gpf_of_13195a() {
        assert_eq!(option2(13195), 29)
    }
}
