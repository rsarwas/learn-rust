//! Squarefree Binomial Coefficients
//!
//! The binomial coefficients nCk can be arranged in triangular form,
//! Pascal's triangle, like this:
//!             
//!                             1
//!                         1		1
//!                     1		2		1
//!                 1		3		3		1
//!             1		4		6		4		1
//!         1		5		10		10		5		1
//!     1		6		15		20		15		6		1
//! 1		7		21		35		35		21		7		1
//!                         .........
//!
//! It can be seen that the first eight rows of Pascal's triangle contain twelve
//! distinct numbers: 1, 2, 3, 4, 5, 6, 7, 10, 15, 20, 21 and 35.
//!
//! A positive integer n is called squarefree if no square of a prime divides n.
//! Of the twelve distinct numbers in the first eight rows of Pascal's triangle,
//! all except 4 and 20 are squarefree. The sum of the distinct squarefree numbers
//! in the first eight rows is 105.
//!
//! Find the sum of the distinct squarefree numbers in the first 51 rows of
//! Pascal's triangle.

use std::collections::HashSet;

/// Problem 203
///
/// Find the sum of the distinct squarefree numbers in the first 51 rows of
/// Pascal's triangle.
pub fn answer() -> u64 {
    sum_square_free_pascal(51) as u64
}

/// Problem 203 (Test Sample)
///
/// Find the sum of the distinct squarefree numbers in the first 8 rows of
/// Pascal's triangle.
#[allow(dead_code)]
pub fn sample() {
    pascal();
    prime_checker();
    println!(
        "Sum of square free numbers in 8 rows of pascals triangle = {}",
        sum_square_free_pascal(8)
    );
}

/// Returns the sum of the distinct squarefree numbers in the first n rows of
/// Pascal's triangle.
///
/// This is a simple brute force solution.  Generate the elements of pascals triangle
/// and if it has not been considered, then check if it is a multiple of any
/// the squared primes less than itself.
///
/// There are only (51 + 1)*51/2 = 26*51 = 1326 elements to check, I can skip all the 1s
/// which reduces this to 1225 elements, which is a very small set of numbers to check.
/// I could impove this further, as the triangle is symetrical, so I could skip the hash
/// lookup by only generating half the triangle, but that complicates things.
///
/// I use the primal::sieve to generate the prime numbers that need to be considered.
/// As an optimization, I create the sieve once, but I need to know the bigest prime I
/// will need.  I could do this by printing pascals triangle and manually selecting the
/// largest number (middle of bottom row).
fn sum_square_free_pascal(n: usize) -> usize {
    // To consider a number only once, we put processed numbers in a hashset
    let mut used: HashSet<usize> = HashSet::new();
    // The largest number in the 51st row is 126410606437752, which is less
    // than 11243248^2, so we will only need primes to 11243248
    let sieve = primal::Sieve::new(11243248);
    let mut sum = 1; // This is for the trivial case of only 1 row
    used.insert(1);
    // The array size must be a constant, limiting generality of this solution
    let mut row: [usize; 51] = [1; 51];
    // skip first two rows, as they are 1 and 1,1, and 1 has already been considered
    // The following loop also skips the leading and trailing 1 in pascals triangle
    for row_n in 3..=n {
        let mut t1 = row[0];
        let mut t2 = row[1];
        for i in 1..row_n - 1 {
            row[i] = t1 + t2;
            if used.insert(row[i]) {
                let mut is_squarefree = true;
                for j in sieve
                    .primes_from(2)
                    .map(|p| p * p)
                    .take_while(|&p| p <= row[i])
                {
                    if row[i] % j == 0 {
                        is_squarefree = false;
                        break;
                    }
                }
                if is_squarefree {
                    sum += row[i];
                }
            }
            t1 = t2;
            t2 = row[i + 1]
        }
    }
    sum
}

/// Prints the first 20 rows of pascals triangle
///
/// Used to test generation algorithm
#[allow(dead_code)]
fn pascal() {
    let n = 20;
    let mut row: [usize; 20] = [1; 20]; //size must be a constant, limiting generality of solution
    println!("{}:  {}", 1, row[0]);
    print!("{}:  {} {}", 2, row[0], row[1]);
    for row_n in 3..=n {
        print!("\n{}: ", row_n);
        print!(" {}", row[0]);
        let mut t1 = row[0];
        let mut t2 = row[1];
        for i in 1..row_n - 1 {
            row[i] = t1 + t2;
            print!(" {}", row[i]);
            t1 = t2;
            t2 = row[i + 1]
        }
        print!(" {}", row[row_n - 1]);
    }
    println!("");
}

/// Prints true/false if the first 20 natural numbers are prime square free
///
/// Used to test algorithm
#[allow(dead_code)]
fn prime_checker() {
    let n = 20;
    let sieve = primal::Sieve::new(n);
    for i in 1..=n {
        println!("Checking {}", i);
        let mut is_squarefree = true;
        for j in sieve.primes_from(2).map(|p| p * p).take_while(|&p| p <= i) {
            println!("Checking prime^2 {}", j);
            if i % j == 0 {
                is_squarefree = false;
                break;
            }
        }
        println!("{} is square free: {}", i, is_squarefree);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn square_free_pascal_test() {
        assert_eq!(sum_square_free_pascal(8), 105);
    }
}
