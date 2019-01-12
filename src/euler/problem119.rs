//! Digit power sum
//!
//! The number 512 is interesting because it is equal to the sum of its digits
//! raised to some power: 5 + 1 + 2 = 8, and 8^3 = 512. Another example of a number
//! with this property is 614656 = 28^4.
//!
//! We shall define an to be the nth term of this sequence and insist that a number
//! must contain at least two digits to have a sum.
//!
//! You are given that a2 = 512 and a10 = 614656.
//! Find a30.

/// Problem 119
///
/// Find a30
pub fn answer() -> u64 {
    option1(30) as u64
}

/// Problem 119 (Test Sample)
///
/// It is a lot faster to search the integer powers than
/// to search all integers.
/// The main problem is that I might need to use very large exponents for the small
/// bases to get numbers that are as large as some of the bigger solutions.
/// in looking at bases from 10..100 and exponents from 2..10, I found more than 30
/// solutions, all smaller than 50^10.  I increased the range of the base, and the
/// range of the exponent, but broke from the loop when I reached 50^10, and got more
/// solutions.  The first 30 solutions all had a base in 7..70 and an exponent in 2..9
#[allow(dead_code)]
pub fn sample() {
    let max = 50_usize.pow(10);
    for i in 6..150_usize {
        for j in 2..100 {
            let p = i.pow(j);
            if sum_of_digits(p) == i {
                println!("{}^{} = {}", i, j, p)
            }
            if p > max {
                break;
            }
        }
    }
}

/// Find a(n) for n = 1..30
///
/// For speed this function limits the search of the base and the exponent
/// per the discussion in the sample code.
pub fn option1(n: usize) -> usize {
    if n > 30 {
        return 0;
    }
    let mut v: Vec<usize> = Vec::with_capacity(40);
    for i in 7..70_usize {
        for j in 2..10 {
            let p = i.pow(j);
            if sum_of_digits(p) == i {
                v.push(p)
            }
        }
    }
    v.sort();
    v[n - 1]
}

/// Returns the sum of a numbers digits
///
/// # Examples
/// ```
/// assert_eq!(sum_of_digits(78), 15)
/// assert_eq!(sum_of_digits(102000), 3)
/// ```
fn sum_of_digits(x: usize) -> usize {
    let mut operand = x;
    let mut solution = 0;
    while operand > 9 {
        solution += operand % 10;
        operand /= 10;
    }
    solution += operand;
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sum_of_digits_test1() {
        assert_eq!(sum_of_digits(4), 4);
    }
    #[test]
    pub fn sum_of_digits_test2() {
        assert_eq!(sum_of_digits(78), 15);
    }
    #[test]
    pub fn sum_of_digits_test3() {
        assert_eq!(sum_of_digits(102000), 3);
    }
    #[test]
    pub fn option1_test1() {
        assert_eq!(option1(2), 512);
    }
    #[test]
    pub fn option1_test2() {
        assert_eq!(option1(10), 614656);
    }
}
