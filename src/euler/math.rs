//! A collection of math functions
//!
//! These are functions that are should be useful to multiple
//! Euler problems.

pub mod fibonacci;

/// Integer Square Root
///
/// See the crate [num_integer] for a more general solution.AsMut
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
