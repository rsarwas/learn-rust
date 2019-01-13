//! Prime square remainders
//!
//! Let pn be the nth prime: 2, 3, 5, 7, 11, ..., and
//! let r be the remainder when (pn−1)^n + (pn+1)^n is divided by pn^2.
//! 
//! For example, when n = 3, p3 = 5, and 43 + 63 = 280 ≡ 5 mod 25.
//! 
//! The least value of n for which the remainder first exceeds 10^9 is 7037.
//! 
//! Find the least value of n for which the remainder first exceeds 10^10.

/// Problem 123
///
/// Find the least value of n for which the remainder first exceeds 10^10
/// Since we know that the solution is above n = 7037, We can start enumerating from there.
pub fn answer() -> u64 {
    option1(7037, 10_usize.pow(10)) as u64
}

/// Problem 123 (Test Sample)
///
/// The least value of n for which the remainder first exceeds 10^9 is 7037.
#[allow(dead_code)]
pub fn sample() {
    println!("pn with rem > 10^4 = {}", option1(1, 10_usize.pow(4)));
}

/// Find n (for the nth prime) with remainder greater than min_rem
///
/// let pn = x an expand/simplify the equation, we will use n = 5 as an example
/// expand 1st term: (x-1)^n = +1x^5 -5x^4 +10x^3 -10x^2  +5x -1 (coef are from pascals triangle with alternating sign)
/// expand 2nd term: (x+1)^n = +1x^5 +5x^4 +10x^3 +10x^2  +5x +1 (coef are from pascals triangle)
/// Add the terms:             +2x^5 +0    +20x^3 +0     +10x +0 (2x the alternating coeficients of pascal)
/// dividing by x^2:           +2x^3       +20x          +10/x
/// This pattern holds for all odd n and the the last term is the only
/// fractional part of the solution, therefore for odd n, the remainder is x^2*10/x = pn*2*P
/// where P is the 2nd to last term of the nth row of pascals triangle;
/// row 0:            1
/// row 1:          1   1
/// row 2:        1   2   1
/// row 3:      1   3   3   1
/// row 4:    1   4   6   4   1
/// row 5:  1   5   10  10  5   1
/// The second to last term in every row is the same as the second, which
/// is always the number of the row. Therefore the remainder for odd n is pn*2*n
/// 
/// If we look at a case where n is even, we get (for example with 4)
///    2(x^4 + 6x^2 +1)/x^2  => remainder = 2
/// since the last term in any row of pascals triangle is always 1, this is true for all even
/// numbered n.
/// 
/// Note if 2n > pn, (i.e. n = 3 pn = 5, 2n = 6, pn*2n = 30) the remainder is not in minimal form
/// however, it is easy to see that after n = 3, 2n is always less than pn
/// n:   1, 2, 3, 4,  5,  6,  7,  8,  9, ...
/// pn:  2, 3, 5, 7, 11, 13, 17, 19, 23, ...
/// 
/// Since we know that the solution is above n = 7037, I can start enumerating from there.
/// 
/// Start must be odd or we will only be checking the even n
/// start should default to 1 (no default parameters in rust); zero is an invalid start
pub fn option1(start: usize, min_rem: usize) -> usize {
    // step by 2 will yield p1, p3, p5, ..); i.e. skips even pn 
    for (p, n) in primal::Primes::all().skip(start-1).zip(start..).step_by(2) {
        let rem = 2*n*p;
        //println!("P{} = {}; pn^2 = {}; rem = {}", n, p, p*p, rem);
        if rem > min_rem {
            return n
        }
    }
    return 0  //We will never get here but compiler doesn't know that
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn prime_square_remainders_test1() {
        assert_eq!(option1(1, 10_usize.pow(9)), 7037);
    }
}
