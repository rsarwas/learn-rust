//! Diophantine reciprocals I
//!
//! In the following equation x, y, and n are positive integers.
//! 
//!      1   1   1
//!      - + - = -
//!      x   y   n
//! 
//! For n = 4 there are exactly three distinct solutions:
//! 
//!      1   1    1
//!      - + -- = -
//!      5   20   4
//! 
//!      1   1    1
//!      - + -- = -
//!      6   12   4
//! 
//!      1   1   1
//!      - + - = -
//!      8   8   4
//! 
//! What is the least value of n for which the number of distinct solutions exceeds one-thousand?
//! 
//! NOTE: This problem is an easier version of Problem 110;
//! it is strongly advised that you solve this one first.

/// Problem 108
///
/// What is the least value of n for which the number of distinct solutions exceeds one-thousand?
///
pub fn answer() -> u64 {
    //option1(1000, 1000)  // takes about 14 seconds (optimized)
    option1(180000, 1000)
}

/// Problem 5 (Test Sample)
///
/// Number of distinct solutions for n
/// 
/// The solution involves considering all x in (n+1..2n), obviously, x = n requires y to be
/// infinite, and x < n requires y to be negative.  x = 2n is always a solution with y = 2n.
/// if x > 2n, then y < 2n, and we may get x,y pairs reversed from solutions already found.
/// solving for y: 1/y = 1/n - 1/x = x/xn - n/nx = (x-n)/xn  => y = xn/(x-n)
/// let x = n + i where i in 1..n => y = (n+i)n/(n+i-n) = (n*n * n*i)/i = n*n/i + n
/// therefore y is an integral solution when n*n is divisible by i
/// i = 1 and i = n are trivial solution.  We could limit the loop and add 2 but this has
/// negligable performance increase.
pub fn diophantine_solutions(n: u64) -> u64 {
    let n2 = n * n;
    (1..=n).filter(|i| {
        n2 % i == 0  
    }).count() as u64
}

/// Prints successive maximum solutions
/// 
/// Originally I started at 4 and stepped by 1, but it became
/// obvious that all the new maximums are multiples of 10.
/// I was unable to dicern any other pattern.  I know the starting
/// point must be above 1000, but I do not by how much.  I'd like
/// to have a hueristic for making a good guess, so I do not need
/// to so many numbers.
#[allow(dead_code)]
pub fn find_trend(n1: u64, n2: u64) {
    let mut max = 0;
    let mut prev = 0;
    for i in (n1..=n2).step_by(10) {
        let solution = diophantine_solutions(i);
        if solution > max {
            println!("{} + {}", i - prev, solution - max);
            max = solution;
            prev = i;
            println!("{} has {} distinct solutions.", i, max);
            if max > 1000 {
                break;
            }
        }
    }
}

/// Find the smallest number above start that has more
/// solutions than goal.
/// 
/// based on the trend analysis, I know if I start with a multiple of
/// 10, I can step by 10.  This invariant is not checked by the code,
/// but enforced by the caller.
fn option1(start: u64, goal: u64) -> u64 {
    let mut answer = 0_u64;
    for i in (start..).step_by(10) {
        let solutions = diophantine_solutions(i);
        if solutions > goal {
            answer = i;
            break
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sample() {
        assert_eq!(diophantine_solutions(4), 3);
    }
}
