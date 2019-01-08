//! Largest palindrome product
//! 
//! A palindromic number reads the same both ways.
//! The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//! 
use super::math;

/// Problem 4 
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// 999 x 999 = 998001, so the largest palindrome possible is 997799
/// Count backwards from n = 997 to 100, create a number p = concat(n, rev(n)).
/// i.e. if n = 978, then p = 978_789
/// for each p try dividing by x in (999..100), stop if divisible (p is the answer),
/// or the result > 999 (p is not a solution).
/// 
/// We could optimize by trying only even x when p is even, but I'm going to guess
/// that the solution is greater than 899, so all p are odd.
pub fn answer() -> u64 {
    // Rust does not have a descending range.  The range (1..n) goes to n-1
    (100..=997).rev()
               .map(create_palindrome)
               .filter(|&x| is_divisible_by_3digits(x))
               .nth(0).unwrap() as u64
}

/// Problem 4 (Test Sample)
///
/// Find the largest palindrome made from the product of two 2-digit numbers.
///
pub fn sample() -> u64 {
    (90..=97).rev()
             .map(create_palindrome)
             .filter(|&x| is_divisible_by_2digits(x))
             .nth(0).unwrap() as u64
}

/// Creates a palindrome from a number
///
/// The output is concat(n, rev(n)) 
/// 
/// # Examples
/// ```
/// assert_eq!(create_palindrome(1234), 12344321)
/// ```
fn create_palindrome(x: usize) -> usize {
    let n = number_of_digits(x);
    x * 10_usize.pow(n) + reverse_number(x)
}

/// Returns a number
/// 
/// # Examples
/// ```
/// assert_eq!(reverse_number(678), 876)
/// ```
fn reverse_number(x: usize) -> usize {
    let n = number_of_digits(x);
    (1..=n)
        .scan(x, |y, _| {
            let r = *y % 10;
            *y /= 10;
            Some(r)
        }) // turns 543 into [3,4,5]
        .fold(0, |a, y| a * 10 + y) //turns [3,4,5] into 345
}

/// Returns the number of digits in a number
/// 
/// # Examples
/// ```
/// assert_eq!(number_of_digits(777), 3)
/// ```
fn number_of_digits(x: usize) -> u32 {
    let mut x = x;
    let mut n = 1;
    while x > 10 {
        x /= 10;
        n += 1;
    }
    n
}

/// Is a number is divisible by two 3 digit numbers?
/// 
/// Returns true if the input is a multiple of
/// any two 3 digit numbers (100..=999).
/// 
/// # Examples
/// ```
/// assert!(is_divisible_by_3digits(856800));
/// assert!(!is_divisible_by_3digits(997799));
/// ```
fn is_divisible_by_3digits(x: usize) -> bool {
    let m = math::isqrt(x);
    (m..1000).rev().any(|y| x % y == 0)
}

/// Is a number is divisible by two 2 digit numbers?
/// 
/// Returns true if the input is a multiple of
/// any two 2 digit numbers (10..=99). 
/// 
/// # Examples
/// ```
/// assert!(is_divisible_by_2digits(9009));
/// assert!(!is_divisible_by_2digits(9119));
/// ```
fn is_divisible_by_2digits(x: usize) -> bool {
    let m = math::isqrt(x);
    (m..100).rev().any(|y| x % y == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_number_of_digits_1() {
        assert_eq!(number_of_digits(0), 1);
    }
    #[test]
    pub fn test_number_of_digits_6() {
        assert_eq!(number_of_digits(12345678), 8);
    }
    #[test]
    pub fn test_reverse_number() {
        assert_eq!(reverse_number(12345678), 87654321);
    }
    #[test]
    pub fn test_is_divisible_by_2digits() {
        assert!(is_divisible_by_2digits(9009));
    }
    #[test]
    pub fn test_is_not_divisible_by_2digits() {
        assert!(!is_divisible_by_2digits(9119));
    }
    #[test]
    pub fn test_is_divisible_by_3digits() {
        assert!(is_divisible_by_3digits(856800));
    }
    #[test]
    pub fn test_is_not_divisible_by_3digits() {
        assert!(!is_divisible_by_3digits(997799));
    }
    #[test]
    pub fn test_palindrome() {
        assert_eq!(create_palindrome(987), 987_789);
    }
    #[test]
    pub fn test_sample() {
        assert_eq!(sample(), 9009);
    }
}
