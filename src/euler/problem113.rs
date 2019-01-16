//! Non-bouncy numbers
//!
//! Working from left-to-right if no digit is exceeded by the digit to its left
//! it is called an increasing number; for example, 134468.
//!
//! Similarly if no digit is exceeded by the digit to its right it is called a
//! decreasing number; for example, 66420.
//!
//! We shall call a positive integer that is neither increasing nor decreasing a
//! "bouncy" number; for example, 155349.
//!
//! As n increases, the proportion of bouncy numbers below n increases such that
//! there are only 12951 numbers below one-million that are not bouncy and only
//! 277032 non-bouncy numbers below 10^10.
//!
//! How many numbers below a googol (10^100) are not bouncy?

/// Problem 113
///
/// How many numbers below a googol (10^100) are not bouncy?
pub fn answer() -> u64 {
    non_bouncy(100) as u64
}

/// Problem 113 (Test Sample)
///
/// Print non-bouncy numbers for numbers below 10^n where n in 1..10
#[allow(dead_code)]
pub fn sample() {
    for i in 1..11 {
        println!("non-bouncy below 10^{} = {}", i, non_bouncy(i));
    }
}

/// Return the number of non-bouncy numbers below 10^exp
///
/// A number from 10^(exp-1) and below 10^exp will have n = exp digits
/// For a number with n digits, there can be between 1 and d
/// unique digits, where d is capped at 9 for increasing (123456789) and
/// 10 for decreasing (9876543210).  As we will show below, the number of
/// increaing and decreasing (non-bouncy) numbers with n digits is:
///
/// ```
///   d:  1          2                3                      9              10
/// inc:  9  + p(2,8)p(1,n-1) + p(3,7)p(2,n-2) + ... + p(9,1)p(8,n-8) +     na
/// dec:  0* + p(2,9)p(1,n-1) + p(3,8)p(2,n-2) + ... + p(9,2)p(8,n-8) + p(10,1)p(9,n-9)
/// ```
///
/// or more generally the count(n) = sum of count(n,d) for d = 0..min(n,10),
/// where count(n,d) is
///
/// * 9 (for d = 1)
/// * [p(d,10-d) + p(d,11-d)] * p(d-1,n+1-d) (for d in 2..9)
/// * p(d,11-d)p(d-1,n+1-d) = p(10,1)p(9,n-9) = p(9,n-9)  (for d = 10)
///
/// Where: the first term is the number of different combinations of d unique digits,
/// e.g: if d = 1 then there is only 1 combination, i.e. all the digits are the same.
/// when d = 2, and increasing, the combinations are:
/// (12, 13, ..., 18, 19,   23, 24, ..., 28, 29,  ...   78, 79,   89) or 8+7+...+2+1
/// combinations. This is the 8th triangle number. When d = 2 and decreasing the combos are:
/// (98, 97, ..., 91, 90,   87, 86, ..., 81, 80,  ...   21, 20,   10) or 9+8+...+2+1
/// combinations. This is the 9th triangle number.  When d = 3 and increasing, the combos are:
/// (123, 124, ... 129,  134, .. 139, .. 178, 179, 189, ... 678, 679, 689, 789).
/// there are (7+6+..2+1)+(6+5+..2+1) + ... + (2+1) + 1 combinations.  This is the sum
/// of the first 7 triangle numbers, this is known as the 7th tetrahedral number.
/// The triangle numbers are so
/// named because the numbers as stone form a triangle in 2 dimensions, the tetrahedrals
/// as stones form a pyramid in 3 dimensions.  This can be generalized to n dimensions,
/// with each number n in dimension r being the sum of the first n numbers in dimension r-1
/// These are called polytonic or figurative numbers, and we denote them p(r,n) where r is
/// the dimension and n is the nth number. following the trend, there will be
/// p(d,10-d) choices for d unique digits when increasing, and p(d,11-d) when decreasing.
///
/// The second term is the number of choices for where the increase from 1 digit to another
/// can occur.  For example in a 5 digit number with 2 unique digits, the choices are:
/// (xyyyy, xxyyy, xxxyy, xxxxy) or 4.  These choices apply to all cominations for 2 digits,
/// both increasing and decreasing.  For a n digit number with 2 unique digits it is clear
/// that there are n-1 choices. (The linearly increasing numbers 1,2,.., n are the polytonic
/// numbers of dimension 1, so the choices are p(1,n-1)). With a 5 digit number with 3 unique digits,
/// the combinations are: (xyzzz, xyyzz, xyyyz,  xxyzz, xxyyz,  xxxyz) or 3+2+1 or the 3rd
/// triangle number. further examination, shows that this pattern holds, and that for
/// a n digit number with 3 unique digits, the choices are p(2,n-2) and in general for a
/// n digit number with d unique digits the choices are p(d-1, n+1-d) for both increasing
/// and decreasing scenarios.
///
/// 0*: when there is only one digit, it is neither increasing or decreasing,
/// I am counting it as increasing, and not counting any decreasing
///
/// # Panics
///
/// will panic if exp is zero or greater than 100
pub fn non_bouncy(exp: usize) -> usize {
    // assert!(exp > 0 && exp < 101);
    // per the formula above, I will need the polytopic numbers p(x,y) where
    // x in 1..9 inclusive and y in 1..(n-x) inclusive.  I can save a lot of
    // effort by calculating these once and caching them in a 9xn-1 matrix
    // note that is matrix has base zero indices, where as my formula are all
    // base 1 indices.  remember to subtract 1 for all matrix accesses.
    // I am also creating a fixed size array because access will be much faster
    // than a variable sized vector on the heap.
    // Also note that these numbers can get very large, and without bounds on
    // size of the matrix, I would need to use a "Big Integer" type.
    let mut p = [[0usize; 99]; 9];
    for i in 0..9 {
        p[i][0] = 1;
    }
    for j in 1..99 {
        p[0][j] = p[0][j - 1] + 1;
    }
    for i in 1..9 {
        for j in 1..99 - i {
            //p[i][j] = super::math::polytopic_number(i + 1, j + 1);
            // super::math::polytopic_number(i,j) is very fast at generating a single number,
            // but since I need to memoize all the numbers, I can use the pattern of
            // of the previously generated numbers
            // 1:   1 2  3  4  5  6
            // 2:   1 3  6 10 15 21
            // 3:   1 4 10 20 35 56
            // 4:   1 5 15 35 91
            // 5:   1 6 21 56
            p[i][j] = p[i][j - 1] + p[i - 1][j];
        }
    }
    // n is the length of the number i.e. 123456 = 6
    // There are 9 non-bouncy nubmers below 10 (n = 1)
    let mut sum: usize = 9; // for n = 1
                            // we start iterating at n = 2
                            // We could save some effort by using the given count for 10^10 and start at n = 11
                            // but this allows us to test our solution.
    for n in 2..=exp {
        // There are always 9 numbers at each n that are not increasing or decreasing
        sum += 9;
        // d is the number of distinct digits in the number i.e. 2244555 = 3
        let d_limit = n.min(10);
        for d in 2..=d_limit {
            let more = if d == 10 {
                // with base 1 index: p(9,n-9)
                p[8][n - 10]
            } else {
                // with base 1-index: [p(d,10-d) + p(d,11-d)] * p(d-1,n+1-d)
                (p[d - 1][9 - d] + p[d - 1][10 - d]) * p[d - 2][n - d] //base 0
            };
            sum += more;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn non_bouncy_test_2() {
        assert_eq!(non_bouncy(2), 99);
    }
    #[test]
    pub fn non_bouncy_test_6() {
        assert_eq!(non_bouncy(6), 12951);
    }
    #[test]
    pub fn non_bouncy_test_10() {
        assert_eq!(non_bouncy(10), 277032);
    }
}
