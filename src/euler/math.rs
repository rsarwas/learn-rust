//! A collection of math functions
//!
//! These are functions that are should be useful to multiple
//! Euler problems.

pub mod fibonacci;

/// Integer Square Root
///
/// See the crate [num_integer] for a more general solution.
///
/// [num_integer]: https://crates.io/crates/num-integer
pub fn isqrt(n: usize) -> usize {
    (n as f64).sqrt() as usize
}

/// Integer division producing both quotient and remainder
///
/// Per the [rust discussion board],
/// this should optimize down to a single asm instruction
///
/// Turns out this is of limited value, since rust cannot decompose
/// a tuple into mutable variables.
/// let (q,r) = div_rem(a,b)  //ok
/// let mut q; let mut r; loop { (q,r) = div_rem(a,b) }  // Not allowed
///
/// [rust discussion board]: https://users.rust-lang.org/t/quotient-and-remainder/16093
#[cfg(unused)]
pub fn div_rem(n: usize, d: usize) -> (usize, usize) {
    let quot = n / d;
    let rem = n % d;
    (quot, rem)
}

/// Returns the nth [polytopic_number] of dimension r
///
/// * When r = 1, these are the linear numbers (1,2,3,4..) P1(n) = n
/// * When r = 2, these are the [Triangle numbers] (1,3,6,10,...) P2(n) = n(n+1)/2
/// * When r = 3, these are the [Tetrahedral numbers] (1,4,10,20,..) P3(n) = n(n+1)(n+2)/(3*2)
///
/// The triangle number n is the sum of the first n linear numbers, the tetrahedral number
/// n is the sum of the first n triangle numbers, etc.
///
/// In general, the r-topic numbers (r-simplex numbers) are:
///   Pr(n) = n(n+1)(n+2)...(n+r-1)/r!
///
/// # Examples
///
/// ```
/// // The 4th triangle number (r=2) is 10
/// assert_eq!(polytopic_number(2, 4), 10)
/// ```
///
/// # Panics
///
/// will panic if r or n is zero
///
/// [polytopic_number]: https://en.wikipedia.org/wiki/Figurate_number
/// [Triangle numbers]: https://en.wikipedia.org/wiki/Triangular_number
/// [Tetrahedral numbers]: https://en.wikipedia.org/wiki/Tetrahedral_number
#[allow(dead_code)]
pub fn polytopic_number(r: usize, n: usize) -> usize {
    // assert!(r > 0 && n > 0);
    let mut numerator = 1;
    let mut denominator = 1;
    for i in 0..r {
        numerator *= n + i;
        denominator *= i + 1;
    }
    numerator / denominator
}

/// Returns a power set of a vector
///
/// A power set is the set (implemeted as a vector) of all the premutations
/// of the members of the input vector. i.e. given the input [a,b,c] the
/// powerset will generate [[], [c], [b], [c, b], [a], [c, a], [b, a], [c, b, a]]
///
/// This solution does not mutate the input vector, and only generates a
/// solution for the slice v[index..].  index must be passed, because
/// rust does not allow passing variable sized slices (it doesn't know the size).
///  
/// This can be used to generate the divisors of a number, as the divisors is
/// the power set of all prime factors.  However due to the cloning of the
/// vectors and the recursive implementation, it will be more economical to
/// implement a less general solution.
#[allow(dead_code)]
pub fn power_set<T: Copy>(v: &[T], index: usize) -> Vec<Vec<T>> {
    if v.len() == index {
        let mut empty = Vec::new();
        empty.push(Vec::new());
        return empty;
    }
    let head = v[index];
    let tail = index + 1;
    let mut ps = power_set(v, tail);
    for i in 0..ps.len() {
        let mut p = ps[i].clone();
        p.push(head);
        ps.push(p);
    }
    ps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn polytopic_number_test() {
        assert_eq!(polytopic_number(2, 4), 10);
    }
}
