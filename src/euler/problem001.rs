//! Multiples of 3 and 5
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5,
//! we get 3, 5, 6 and 9. The sum of these multiples is 23.
//! Find the sum of all the multiples of 3 or 5 below 1000.

/// Euler Problem # 1
/// 
/// Find the sum of all the multiples of 3 or 5 below 1000.
pub fn answer() -> u64 {
	option4(1000)
}

/// Euler Problem # 1 (Test Sample)
/// 
/// Find the sum of all the multiples of 3 or 5 below 10.
#[allow(dead_code)]
pub fn sample() {
    println!("Option1(10) = {}", option1(10));
    println!("Option2(10) = {}", option2(10));
    println!("Option3(10) = {}", option3(10));
    println!("Option4(10) = {}", option4(10));
}

/// Find the sum of all the multiples of 3 or 5 below n.
///
/// Option #1: Brute force: simple loop, check every number below n
/// This is a slow solution: debug run time ~50µs; release run time ~1300ns
///
/// # Examples
///
/// ```
/// let answer = option1(10);
/// asert_eq!(answer, 23);
/// ```
fn option1(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

/// Find the sum of all the multiples of 3 or 5 below n.
///
/// Option #2: This uses 1 iterator. It looks at all numbers and
/// filters out the multiples before summing.
/// This is the simplest solution, but also the slowest:
/// debug run time ~75µs; release run time ~1900ns
///
/// # Examples
///
/// ```
/// let answer = option2(10);
/// asert_eq!(answer, 23);
/// ```
fn option2(n: u64) -> u64 {
    (3..n).filter(|&x| x % 3 == 0 || x % 5 == 0).sum()
}

/// Find the sum of all the multiples of 3 or 5 below n.
///
/// Option #3: This uses 2 iterators. First sum the multiples of three,
/// then sum the multiples of 5 after filtering out the multpiles of 3
/// (they have already been counted).  The filter slows this down a little.
/// Almost the fastest solution: debug run time ~30µs; release run time ~600ns
///
/// # Examples
///
/// ```
/// let answer = option3(10);
/// asert_eq!(answer, 23);
/// ```

fn option3(n: i32) -> i32 {
    let sum3: i32 = (3..n).step_by(3).sum();
    let sum5: i32 = (5..n).step_by(5).filter(|&x| x % 3 != 0).sum();
    sum3 + sum5
}

/// Find the sum of all the multiples of 3 or 5 below n.
///
/// Option #4: This uses 3 iterators. First sum the multiples of three,
/// then the multiples of 5, then subtract the multpiles of 15.
/// This is the fastest solution: debug run time ~28µs; release run time ~500ns
///
/// # Examples
///
/// ```
/// let answer = option4(10);
/// asert_eq!(answer, 23);
/// ```

fn option4(n: u64) -> u64 {
    let sum3: u64 = (3..n).step_by(3).sum();
    let sum5: u64 = (5..n).step_by(5).sum();
    let sum15: u64 = (15..n).step_by(15).sum();
    sum3 + sum5 - sum15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_loop() {
        assert_eq!(option1(10), 23);
    }
    #[test]
    pub fn test_1_iterator() {
        assert_eq!(option2(10), 23);
    }
    #[test]
    pub fn test_2_iterators() {
        assert_eq!(option3(10), 23);
    }
    #[test]
    pub fn test_3_iterators() {
        assert_eq!(option4(10), 23);
    }
}
