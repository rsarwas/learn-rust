//! Matrix Sum
//!
//! We define the Matrix Sum of a matrix as the maximum sum of matrix elements
//! with each element being the only one in his row and column. For example,
//! the Matrix Sum of the matrix below equals 3315 ( = 863 + 383 + 343 + 959 + 767):
//!
//!   7  53 183 439 863
//! 497 383 563  79 973
//! 287  63 343 169 583
//! 627 343 773 959 943
//! 767 473 103 699 303
//!

const SAMPLE_N: usize = 5;
const SAMPLE_N2: usize = 25;
const N: usize = 15;
const N2: usize = 225;
const ULIMIT: usize = 1000; // A number bigger than all the matrix elements

/// Problem 345
///
/// Find the Matrix Sum of: the matrix in ../problem345.txt
pub fn answer() -> u64 {
    let n = N;
    let max = max_assignment(&mut matrix(), n).unwrap();
    sum_of_assignments(&matrix(), n, max) as u64
}

/// Problem 345 (Test Sample)
///
/// Find the Matrix Sum of: the sample matrix above
#[allow(dead_code)]
pub fn sample() {
    let n = SAMPLE_N;
    let mut m = sample_matrix();
    println!("original matrix");
    print_matrix(&m, n);
    let max = max_assignment(&mut m, n).unwrap();
    println!("modified matrix:");
    print_matrix(&m, n);
    //println!("assign workers:");
    //min_several(&mut m, n);
    //println!("minimized matrix:");
    //print_matrix(&m, n);
    //println!("max assignments = {:?}", max);
    let sum = sum_of_assignments(&sample_matrix(), n, max);
    println!("Sum of max assignents = {}", sum);
}

/// Returns the maximum assignments for a matrix
///
/// This is done by subtracting each element from a
/// value known to be greater than each cell, and then
/// finding the minimal assignments for that matrix.
fn max_assignment(m: &mut [usize], n: usize) -> Option<Vec<(usize, usize)>> {
    // convert to a matrix that can be minimized
    for i in 0..n * n {
        m[i] = ULIMIT - m[i];
    }
    assignment(m, n)
}

/// Returns the minimum assignments for a matrix
///
/// This uses the [hungarian algorithm] to the
/// [assignment problem].
///
/// [hungarian algorithm]: https://en.wikipedia.org/wiki/Hungarian_algorithm
/// [assignment problem]: https://en.wikipedia.org/wiki/Assignment_problem
fn assignment(m: &mut [usize], n: usize) -> Option<Vec<(usize, usize)>> {
    // minimize rows (step 1)
    for r in 0..n {
        let mut min = ULIMIT;
        let base = r * n;
        for i in base..base + n {
            if m[i] < min {
                min = m[i];
            }
        }
        for i in base..base + n {
            m[i] = m[i] - min;
        }
    }
    // minimize columns (step 2)
    for c in 0..n {
        let mut min = ULIMIT;
        for r in 0..n {
            let i = r * n + c;
            if m[i] < min {
                min = m[i];
            }
        }
        for r in 0..n {
            let i = r * n + c;
            m[i] = m[i] - min;
        }
    }
    //println!("modified matrix:");
    //print_matrix(&m, n);

    // repeat steps 3 and 4 until there is an assignment
    let mut s = assign_workers(&m, n);
    while let Err((rows, cols)) = s {
        //println!("Undone:");
        //println!("  marked rows: {:?}", rows);
        //println!("  marked cols: {:?}", cols);
        //println!("  updated matrix:");
        update_cost(m, n, &rows, &cols);
        //print_matrix(&m, n);
        s = assign_workers(&m, n);
    }
    if let Ok(assignments) = s {
        //println!("Done: {:?}", assignments);
        //print_matrix(&m, n);
        return Some(assignments);
    } else {
        println!("WTF???");
    }
    None
}

/// Assign workers to tasks
///
/// Returns either a list of assignments, or a pair of
/// masks for the rows/columns that are marked 'out' for
/// the next iteration.
///
/// This tasks looks are each row (workers), and if there is
/// an optimal task (col with zero), then it calls that an
/// assignment, and crosses out any other optimal assignments
/// in that column and row.  Since it is unclear which task to
/// pick when there are two (or more) columns with zeros, and
/// picking the wrong one could lead to a sub optimal list of
/// assignments (i.e. a may 'steal' a task from a worker that
/// has no other optimal assignment), I will go through the list
/// of rows twice, first picking all of the workers with a single
/// optimal assignment, and then hopefully the rest will not lead
/// to any sub-optimal solutions.
fn assign_workers(m: &[usize], n: usize) -> Result<Vec<(usize, usize)>, ([bool; N], [bool; N])> {
    // All of the zero values that should not be ignored
    let mut crossed_out: [bool; N2] = [false; N2];
    // A list of the rows that are skipped on the first round
    let mut revisit = Vec::with_capacity(n);
    // A list of row/col pairs that are the location of an optimal assignment
    let mut assignments = Vec::with_capacity(n);
    // A list of rows with no assignments
    let mut no_assignments = Vec::with_capacity(n);

    // find singlular assignments in all rows
    for r in 0..n {
        //println!("....checking row {}", r);
        let mut zero_count = 0;
        let mut found = (n, n);
        for c in 0..n {
            let i = r * n + c;
            if m[i] == 0 && !crossed_out[i] {
                zero_count += 1;
                found = (r, c);
                if zero_count > 1 {
                    //println!("....skipping row {}", r);
                    revisit.push(r);
                    break;
                }
            }
        }
        if zero_count == 0 {
            //println!("....no assignment row {}", r);
            no_assignments.push(r);
        }
        if zero_count == 1 {
            assignments.push(found);
            // clear column;  there are no zeros in row to clear
            //println!("....assignment found at: {:?}", found);
            for r2 in 0..n {
                let i = r2 * n + found.1;
                if r2 != found.0 && m[i] == 0 && !crossed_out[i] {
                    //println!("....crossing out: ({},{})", r2, found.1);
                    crossed_out[i] = true;
                }
            }
        }
    }
    for &r in revisit.iter() {
        //println!("....re-checking row {}", r);
        // This is similar to above, but we just take the first zero in the
        // row and cross out the other zeros in the rows/columns
        let mut unassigned_row = true;
        for c in 0..n {
            let i = r * n + c;
            if m[i] == 0 && !crossed_out[i] {
                let found = (r, c);
                //println!("....assignment found at: {:?}", found);
                unassigned_row = false;
                assignments.push(found);
                // clear the rest of the row
                // TODO: is this necessary?
                for c2 in c + 1..n {
                    let i = r * n + c2;
                    if m[i] == 0 && !crossed_out[i] {
                        //println!("....crossing out: ({},{})", r, c2);
                        crossed_out[i] = true;
                    }
                }
                // clear column;
                for r2 in 0..n {
                    let i = r2 * n + found.1;
                    if r != found.0 && m[i] == 0 && !crossed_out[i] {
                        //println!("....crossing out: ({},{})", r2, found.1);
                        crossed_out[i] = true;
                    }
                }
                break;
            }
        }
        if unassigned_row {
            //println!("....no assignment row {}", r);
            no_assignments.push(r);
        }
    }
    if assignments.len() < n {
        // Step 3 the drawing part
        // Create and return the masking arrays
        let mut row_marked_at: [bool; N] = [false; N];
        let mut col_marked_at: [bool; N] = [false; N];
        // For all rows without an assignment:
        //   * Mark the row.
        //   * Mark all columns having zeros in the row.
        //   * Mark all rows having an assignment in newly marked columns.
        //     (I need to consider all newly marked rows just like those without an assignment.
        //      I will do this by adding them to the no assignment vector)
        //println!("....no assignment rows {:?}", no_assignments);
        while let Some(r) = no_assignments.pop() {
            //println!("....marking row {}", r);
            row_marked_at[r] = true;
            for c in 0..n {
                if m[r * n + c] == 0 {
                    //println!("....marking col {}", c);
                    col_marked_at[c] = true;
                    for (r2, _) in assignments.iter().filter(|p| p.1 == c) {
                        if !row_marked_at[*r2] {
                            //println!("....marking row (from column) {}", r2);
                            row_marked_at[*r2] = true;
                            no_assignments.push(*r2);
                        }
                    }
                }
            }
        }
        // Now draw lines through all marked columns and **unmarked** rows.
        for i in 0..n {
            row_marked_at[i] = !row_marked_at[i];
        }
        return Err((row_marked_at, col_marked_at));
    }
    // each worker has an assignment, so we are done.
    Ok(assignments)
}

/// Update the cost matrix (step 4)
///
/// Find the second minimum cost among the remaining choices,
/// and update the costs
fn update_cost(m: &mut [usize], n: usize, row_marked_at: &[bool], col_marked_at: &[bool]) {
    // Find the min value in unmarked area
    let mut min = ULIMIT;
    for r in 0..n {
        if row_marked_at[r] {
            continue;
        }
        for c in 0..n {
            if col_marked_at[c] {
                continue;
            }
            let i = r * n + c;
            if m[i] < min {
                min = m[i];
            }
        }
    }

    // Subtract this value from every unmarked element
    // and add it to every element marked by a row and a column
    for r in 0..n {
        if row_marked_at[r] {
            for c in 0..n {
                if col_marked_at[c] {
                    let i = r * n + c;
                    m[i] += min;
                }
            }
        } else {
            for c in 0..n {
                if !col_marked_at[c] {
                    let i = r * n + c;
                    m[i] -= min;
                }
            }
        }
    }
}

/// Returns the sum of the assignment cells
///
/// Uses the (row,column) indexes in the 'assign' vector to get the
/// cell value from the original matrix.
fn sum_of_assignments(m: &[usize], n: usize, assign: Vec<(usize, usize)>) -> usize {
    assign.iter().map(|x| m[x.0 * n + x.1]).sum()
}

/// Returns the smaple matrix for testing
///
/// The 5x5 matrix is a single array of 25 usize elements
fn sample_matrix() -> [usize; SAMPLE_N2] {
    [
          7,  53, 183, 439, 863,
        497, 383, 563,  79, 973,
        287,  63, 343, 169, 583,
        627, 343, 773, 959, 943,
        767, 473, 103, 699, 303,
    ]
}

/// Returns the problem matrix
///
/// The 15x15 matrix is a single array of 225 usize elements
fn matrix() -> [usize; N2] {
    [
          7,  53, 183, 439, 863, 497, 383, 563,  79, 973, 287,  63, 343, 169, 583,
        627, 343, 773, 959, 943, 767, 473, 103, 699, 303, 957, 703, 583, 639, 913,
        447, 283, 463,  29,  23, 487, 463, 993, 119, 883, 327, 493, 423, 159, 743,
        217, 623,   3, 399, 853, 407, 103, 983,  89, 463, 290, 516, 212, 462, 350,
        960, 376, 682, 962, 300, 780, 486, 502, 912, 800, 250, 346, 172, 812, 350,
        870, 456, 192, 162, 593, 473, 915,  45, 989, 873, 823, 965, 425, 329, 803,
        973, 965, 905, 919, 133, 673, 665, 235, 509, 613, 673, 815, 165, 992, 326,
        322, 148, 972, 962, 286, 255, 941, 541, 265, 323, 925, 281, 601,  95, 973,
        445, 721,  11, 525, 473,  65, 511, 164, 138, 672,  18, 428, 154, 448, 848,
        414, 456, 310, 312, 798, 104, 566, 520, 302, 248, 694, 976, 430, 392, 198,
        184, 829, 373, 181, 631, 101, 969, 613, 840, 740, 778, 458, 284, 760, 390,
        821, 461, 843, 513,  17, 901, 711, 993, 293, 157, 274,  94, 192, 156, 574,
         34, 124,   4, 878, 450, 476, 712, 914, 838, 669, 875, 299, 823, 329, 699,
        815, 559, 813, 459, 522, 788, 168, 586, 966, 232, 308, 833, 251, 631, 107,
        813, 883, 451, 509, 615,  77, 281, 613, 459, 205, 380, 274, 302,  35, 805,
    ]
}

/// Pretty prints and array as a matrix
///
/// using the fmt::Debug trait doesn't work for the large array,
/// and it is hard to see what is going on without displaying in
/// rows and columns
fn print_matrix(m: &[usize], n: usize) {
    for r in 0..n {
        for c in 0..n {
            let i = r * n + c;
            print!("{:>4}", m[i]);
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn matrix_sum_test() {
        let n = SAMPLE_N;
        let max = max_assignment(&mut sample_matrix(), n).unwrap();
        let sum = sum_of_assignments(&sample_matrix(), n, max);
        assert_eq!(sum, 3315);
    }
}
