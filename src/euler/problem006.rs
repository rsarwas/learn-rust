//! Sum square difference
//!
//! The sum of the squares of the first ten natural numbers is,
//!
//!            1^2 + 2^2 + ... + 10^2 = 385
//!
//! The square of the sum of the first ten natural numbers is,
//!
//!            (1 + 2 + ... + 10)^2 = 552 = 3025
//!
//! Hence the difference between the sum of the squares of the first ten natural
//! numbers and the square of the sum is 3025 − 385 = 2640.
//!
//! Find the difference between the sum of the squares of the first one hundred
//! natural numbers and the square of the sum.

/// Problem 6
///
/// Find the difference between the sum of the squares of the first one hundred
/// natural numbers and the square of the sum.
///
pub fn answer() -> u64 {
    option1(100) as u64
}

/// Problem 5 (Test Sample)
///
/// Find the difference between the sum of the squares of the first ten
/// natural numbers and the square of the sum.
#[allow(dead_code)]
pub fn sample() -> u64 {
    option1(10) as u64
}

/// Difference between the sum of the squares of the first n
/// natural numbers and the square of the sum.
///
fn option1(n: usize) -> usize {
    let sum_of_squares = (1..=n).map(|x| x*x ).sum::<usize>();
    //let sum = (1..=10).sum::<usize>();
    let sum = sum_1_to(n);
    let square_of_sum = sum * sum;
    square_of_sum - sum_of_squares
}

/// Returns the sum of the first n natural numbers
/// 
/// i.e. 1..6 => (1+6)+(2+5)+(3+4) = 7*3 = (6+1)*6/2 = 21
/// 1..5 => (1+5)+(2+4)+(3) = 6*2.5 = (5+1)*5/2 = 3*5 = 15
fn sum_1_to(n: usize) -> usize {
    (n + 1) * n / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sample() {
        assert_eq!(sample(), 2640);
    }
}
