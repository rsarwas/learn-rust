//! Prime generating integers
//!
//! Consider the divisors of 30: 1,2,3,5,6,10,15,30.
//! It can be seen that for every divisor d of 30, d+30/d is prime.
//!
//! Find the sum of all positive integers n not exceeding 100 000 000
//! such that for every divisor d of n, d+n/d is prime.

use super::math;
use std::collections::HashSet;

/// Problem 357
///
/// Find the sum of all positive integers n not exceeding 100 000 000
/// such that for every divisor d of n, d+n/d is prime.
pub fn answer() -> u64 {
    sum_divisors_to(100_000_000) as u64
}

/// Problem 357 (Test Sample)
///
/// Test the finding of divisors of n, and the prime check of d + n/d
#[allow(dead_code)]
pub fn sample() {
    let n = 100;
    let sieve = primal::Sieve::new(101); //Add 1 to get to check if 1+n/1 is prime
    for i in 1..n {
        println!("Divisors of {}: {:?}", i, divisors(i, &sieve));
        println!("Unique Divisors of {}: {:?}", i, unique_divisors(i, &sieve));
        println!(
            "Min Divisors of {}: {:?}",
            i,
            divisors_min(i, math::isqrt(i), &sieve)
        );
        println!(
            "Divisors of {} are prime: {}",
            i,
            divisors_are_prime(i, &sieve)
        );
    }
    println!("Sum of n with prime divisors = {}", sum_divisors_to(n));
    //println!("divisors of 12 = {:?}", divisors(12, &sieve));
    //println!("power set of [1,2,3] = {:?}", math::power_set(&vec![1, 2, 3], 0));
}

/// Returns the sum of all positive integers not exceeding n
/// such that for every divisor d of n, d+n/d is prime.
///
/// All numbers are divisible by 1 and n, ∴ d + n/d = 1+n and n + n/n = n+1.
/// If n is odd, then n+1 is even and not prime.  ∴ we know that all odd n will
/// not be included in the sum, and we can limit the search space to just the even
/// numbers (and 1)
///
/// Since every divisor has a "match"  a*b = n, where a = n/b and b = n/a,
/// we know that d + n/d = a + (a*b)/a = a+b and b + (a*b)/b = b + a, we only
/// need to check the lower (or upper) half of the divisors.
///
/// If a number is a square, then r^2 = n, so r is a divisor and r + n/r = r + r
/// since 2r is not prime, all square numbers will not be part of the sum
///
/// If a number is divisible by 4, then 2 is a divisor, since 2 is a divisor of 4.
/// if n = 4*a = 2*2*a d=n/d = 2 + (2*2*a)/2 = 2 + 2a = 2(a+1).  Since 2(a+1) is not
/// prime for any a, all numbers divisible by 4 (or a multiple of 4) will not be part
/// of the sum. A similar analysis can be done for all squares, and we can conclude
/// that any number that is a multiple of a square number is not part of the sum.
///
/// By using the fact that 1+n/1 must be prime for n to be considered, I can limit
/// the search space considerably by only considering n that are a prime-1.  This
/// is a much smaller search space than the even numbers.
///
/// For comparison solution takes:
///   450 sec searching even numbers and including multiples of squares
///   145 sec searching even numbers and excluding multiples of squares
///    59 sec searching primes-1 and including multiples of squares
///    16 sec searching primes-1 and excluding multiples of squares
///
/// All those numbers used a hashset to create a unique list of divisors.
/// It turns out that processing duplicate divisors, is faster (9sec) than
/// using a hashset (16sec) to ensure we have a unique set of divisors.
///
/// This solution can still be improved by using a non-recursive algorithm to generate
/// a unique list of just the small divisors of a number (excluding 1)
/// 1 can be ignored because 1 and n are always valid since we pick 1 + n to be prime
/// for the other divisors, if d+n/d = d'+n/d' where d' is the larger divisor that
/// matchs d, i.e. d' = n/d, so the larger divisors do not need to be checked.
///
/// The vast majority of the time in this solution is spent in the
/// divisors_are_prime() method.  The solution time drops to 45ms if that method
/// simply returns true.  I expect that the divisor generator is the bulk of the
/// time in that method.
pub fn sum_divisors_to(n: usize) -> usize {
    // The sieve will be used for an is_prime() check.  That method will panic
    // if n is greater than the upper_bound provide when creating the sieve.
    // It seems like we would only need primes to sqrt(n) to check n (similar to
    // generating factors, but that is not the case.)
    // If we are looking at all even numbers, then we must check 1+n/1.  e.g.
    // 34 has divisors 1,2,17,34 and 19 is prime, but 35 is not.
    // If we are looking at only the prime numbers, then the
    // If we are looking at n = p-1 then we will use the sieve to generate the
    // primes to n+1.  Therefore in both cases, we need a large sieve.

    let sieve = primal::Sieve::new(n + 1);

    // stupid solution
    //(1..=n).filter(|&d| divisors_are_prime(d, &sieve)).sum()

    //I can cut the search space in half by only looking at the even numbers,
    //If n is odd, then 1 + n/1 is even, and not prime (except when n = 1)
    //(2..=n).step_by(2).filter(|&d| divisors_are_prime(d, &sieve)).sum()

    // Using the fact that 1+n/1 must be prime for n to be considered, I can limit the search
    // space considerably by only considering n that are a prime-1
    // for this I will need a larger sieve.
    sieve
        .primes_from(2)
        .map(|p| p - 1)
        .take_while(|&x| x <= n)
        .filter(|&d| divisors_are_prime(d, &sieve))
        .sum()
}

/// Returns true if all divisors d of n, satisfy d+n/d is prime
///
/// This is a simple brute force algorithm.
///
/// The prime seive is used to get the prime factors for n.  
/// I only need to look at the first half of the divisors, due to symmetry:
/// for divisors of m {1, d2 ... dn-1, m}; 1 + m/1 == m + m/m; similarly for d2 and dn-1 etc.
///
/// If there is an odd number of divisors, then the middle divisor is a square root and this number can be ignored
/// because n/s = s and s + n/s = 2s which is not prime
///
/// I can ignore all odd numbers (except 1) as 1 + n/1 is even and therefore not prime
/// Similarly I can ignore all multiples of 4, 9, 16, ....
/// Unfortunately, initial testing revealed that this approach is not nearly fast enough
///   10 000 < .5sec, while 100 000 > 10sec at best this implies 100 000 000 > 80000sec
///   even compiled, the code takes 22s for 10^6
pub fn divisors_are_prime(n: usize, sieve: &primal::Sieve) -> bool {
    // skip the following checks to save time, caller must ensure all n are even
    /*
    // n == 1 is a special case
    if n == 1 {
        return true;
    }
    // Odd numbers are guaranteed to fail
    if n % 2 == 1 {
        return false;
    }
    */
    // We know multiples of the squares (4,9,25,...) will fail, see proof above.
    // Adding this test takes time on each n, but finding the divisors is significantly
    // slower, so this saves time overall
    let root_n = math::isqrt(n);
    for i in 2..root_n {
        let square = i * i;
        if square > root_n {
            break;
        }
        if n % square == 0 {
            return false;
        }
    }
    // Check only the lower divisors
    // turns out this is slower than a filter on the iterator
    // divisors_min(n, root_n, sieve).iter().all(|&d| sieve.is_prime(d + n/d))

    // If I am checking all even numbers, then I need to check 1+n/1
    //divisors(n, sieve).iter().filter(|&x| x <= &root_n).all(|&d| sieve.is_prime(d + n/d))

    // If I already know that n+1 is prime, then I can skip the check of 1 + n/1
    // This filter reduces the number of primal checks and potentially the size of the sieve
    // but beware sieve to sqrt(n) is not adequate.  e.g need to check is_prime(2 + n/2)
    divisors(n, sieve)
        .iter()
        .filter(|x| **x > 1 && **x <= root_n)
        .all(|&d| sieve.is_prime(d + n / d))
}

/// Returns just the "small" divisors of a number
///
/// e.g.  All the divisors of 12 are 1,2,3,4,6,12. This will return
/// just 1,2,3.  The remaining divisors can be generated from these.
///
/// Unfortunately to do this we generate all the divisors, and then need to
/// clone them into a new vector which is less efficient than filtering the
/// full divisor list on the fly.
fn divisors_min(n: usize, root_n: usize, sieve: &primal::Sieve) -> Vec<usize> {
    let d = divisors(n, sieve);
    d.iter().cloned().filter(|&x| x <= root_n).collect()
}

/// Returns all the divisors of n in a hashset
///
/// The hashset guarantees the divisors are unique, but not sorted.
///
/// The sieve is used to get the prime factors of n.
/// The sieve returns (pi, ci) tuples where ci is the number of
/// times pi is a factor of n.  This is expanded into a vector
/// i.e. 36 => [(2,2),(3,2)] => [2,2,3,3].  A recursive "powerset"
/// method is used to generate all the divisors from this vector,
fn unique_divisors(n: usize, sieve: &primal::Sieve) -> HashSet<usize> {
    let factors = sieve.factor(n).unwrap();
    let mut all_factors: Vec<usize> = Vec::new();
    for (p, c) in factors.iter() {
        for _ in 0..*c {
            all_factors.push(*p);
        }
    }
    unique_divisors_recursive(&mut all_factors)
}

/// Returns all the divisors of n in a hashset
///
/// The hashset guarantees the divisors are unique, but not sorted.
///
/// The input is a vector of all the prime factors, A recursive "powerset"
/// algorithm is used to generate all the permutations of this vector,
/// i.e. [2,2,3,3] => [[], [2], [3], [2*2], [2*3], [3*3], [2*2*3], [2*3*3], [2*2*3*3]]
/// to generate all the divisors [1,2,3,4,6,9,12,18,36]
fn unique_divisors_recursive(v: &mut Vec<usize>) -> HashSet<usize> {
    if v.is_empty() {
        return [1].iter().cloned().collect();
    }
    let head = v.pop().unwrap();
    let ps = unique_divisors_recursive(v);
    let ps2: HashSet<usize> = ps.iter().map(|x| x * head).collect();
    let union: HashSet<usize> = ps2.union(&ps).cloned().collect();
    union
}

/// Returns all the divisors of n in a sorted vector
///
/// A given divisor may be listed multiple times in the vector.
/// With the powerset algorithm, the factors 2,3 and 3,2 will
/// both yield a divisor of 6.
///
/// The sieve is used to get the prime factors of n.
/// The sieve returns (pi, ci) tuples where ci is the number of
/// times pi is a factor of n.  This is expanded into a vector
/// i.e. 12 => [(2,2),(3,1)] => [2,2,3].  A recursive "powerset"
/// method is used to generate all the divisors of this vector.
fn divisors(n: usize, sieve: &primal::Sieve) -> Vec<usize> {
    let factors = sieve.factor(n).unwrap();
    let mut all_factors: Vec<usize> = Vec::new();
    for (p, c) in factors.iter() {
        for _ in 0..*c {
            all_factors.push(*p);
        }
    }
    divisors_recursive(&mut all_factors)
}

/// Returns all the divisors of n in a sorted vector
///
/// The divisors are sorted largest to smallest; there may be duplicates.
///
/// The input is a vector of all the prime factors, A recursive "powerset"
/// algorithm is used to generate all the permutations of this vector,
/// i.e. [2,2,3] => [[], [2], [2], [3], [2*2], [2*3], [2*3], [2*2*3]]
/// to generate all the divisors, i.e [1,2,2,3,4,6,6,12]
fn divisors_recursive(v: &mut Vec<usize>) -> Vec<usize> {
    if v.is_empty() {
        let mut empty = Vec::new();
        empty.push(1);
        return empty;
    }
    let head = v.pop().unwrap();
    let mut ps = divisors_recursive(v);
    let mut ps2: Vec<usize> = ps.iter().map(|x| x * head).collect();
    ps2.append(&mut ps);
    ps2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn non_bouncy_test_2() {
        assert_eq!(sum_divisors_to(100), 401);
    }
}
