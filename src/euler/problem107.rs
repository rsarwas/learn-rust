//! Minimal network
//!
//! The following undirected network consists of seven vertices
//! and twelve edges with a total weight of 243.
//!
//!     {illustration elided}
//!
//! The same network can be represented by the matrix below.
//!
//!     	A	B	C	D	E	F	G
//!    A	-	16	12	21	-	-	-
//!    B	16	-	-	17	20	-	-
//!    C	12	-	-	28	-	31	-
//!    D	21	17	28	-	18	19	23
//!    E	-	20	-	18	-	-	11
//!    F	-	-	31	19	-	-	27
//!    G	-	-	-	23	11	27	-
//!
//! However, it is possible to optimise the network by removing some edges
//! and still ensure that all points on the network remain connected.
//! The network which achieves the maximum saving is shown below.
//! It has a weight of 93, representing a saving of 243 − 93 = 150 from
//! the original network.
//!
//!     	A	B	C	D	E	F	G
//!    A	-	16	12	-	-	-	-
//!    B	16	-	-	17	-	-	-
//!    C	12	-	-	-	-	-	-
//!    D	-	17	-	-	18	19	-
//!    E	-	-	-	18	-	-	11
//!    F	-	-	-	19	-	-	-
//!    G	-	-	-	-	11	-	-
//!
//! Using network.txt (right click and 'Save Link/Target As...'), a 6K text file
//! containing a network with forty vertices, and given in matrix form, find the
//! maximum saving which can be achieved by removing redundant edges whilst ensuring
//! that the network remains connected.

use std::collections::HashSet;
use std::fs;

/// Problem 107
///
/// Find the maximum savings by removing redundant edges in problem107.txt
pub fn answer() -> u64 {
    let filename = "src/problem107.txt";
    let contents = fs::read_to_string(filename).expect("failed to read the file");
    let n = 40;
    let v = contents
        .split(|c: char| c.is_whitespace() || c == ',')
        .map(|x| x.parse().ok())
        .collect();
    let w1 = weight(&v, n);
    let w2 = prim(&v, n);
    (w1 - w2) as u64
}

/// Problem 107 (Test Sample)
///
/// Find the maximum savings by removing redundant edges in problem107.txt
#[allow(dead_code)]
pub fn sample() {
    let (n, v) = sample_network();
    println!("{:?}", v);
    let w1 = weight(&v, n);
    let w2 = prim(&v, n);
    println!(
        "old weight = {}, new weight = {}, savings = {}",
        w1,
        w2,
        w1 - w2
    );
}

/// Returns the vector (7x7 matrix) of the sample network
///
fn sample_network() -> (usize, Vec<Option<usize>>) {
    let n = 7;
    let a:[usize; 49] = [
         0, 16, 12, 21,  0,  0,  0,
        16,  0,  0, 17, 20,  0,  0,
        12,  0,  0, 28,  0, 31,  0,
        21, 17, 28,  0, 18, 19, 23,
         0, 20,  0, 18,  0,  0, 11,
         0,  0, 31, 19,  0,  0, 27,
         0,  0,  0, 23, 11, 27,  0,
    ];
    let v = a
        .iter()
        .map(|&x| if x == 0 { None } else { Some(x) })
        .collect();
    (n, v)
}

/// Weight of Minimal Spanning Tree
///
/// Takes a reference to a vector of size nxn with weights for edge (i,j),
/// Where i,j are in (0..n) and represent the nodes (vertices) in the graph.
/// The weights are Optional<usize>, since not all edges are valid.
/// The caller must ensure that the matrix is valid, i.e. (i,i) == None,
/// and that matrix M from the vector equals the transpose of M.
///
/// Prim's algorithm (https://en.wikipedia.org/wiki/Prim%27s_algorithm)
/// is used.  In simple terms, All vertices start in the set S (unpicked),
/// An arbitrary vertex is removed from S and added to S' (picked).
/// loop until S is empty
///    find the smallest weight of all the edges between S and S'
///    remove the associated vertex from S and add it to S'
///    Add the weight to a totalizer, and/or add the edge to a set E
/// The set E is the minimal spanning tree, and the total is the minimal
/// weight.
fn prim(m: &Vec<Option<usize>>, n: usize) -> usize {
    let mut total = 0;
    // r = the index of a matrix row, or the id of a vertex that has been picked,
    //     these are the rows that will be searched
    let mut r: HashSet<usize> = HashSet::with_capacity(n);
    // c = the index of a matrix column, or the id of a vertex that has NOT been picked
    //     these are the columns that will be searched
    let mut c: HashSet<usize> = (1..n).collect();
    r.insert(0);
    while !c.is_empty() {
        let (ci, w) = find_minimum(&m, &r, &c, n);
        total += w;
        c.remove(&ci);
        r.insert(ci);
    }
    total
}

/// Returns the minimal edge in M by connecting a vertix in r with a vertex in c
///
/// The weights in m are assumed to be unique, However, if they are not, the
/// algorithm should still work as long as one of the minimal weights is choosen.
fn find_minimum(
    m: &Vec<Option<usize>>,
    r: &HashSet<usize>,
    c: &HashSet<usize>,
    n: usize,
) -> (usize, usize) {
    let mut i_sel: usize = 0;
    let mut w_min: usize = usize::max_value();
    for &ri in r {
        for &ci in c {
            let i = ri * n + ci;
            if let Some(w) = m[i] {
                if w < w_min {
                    w_min = w;
                    i_sel = ci;
                }
            }
        }
    }
    (i_sel, w_min)
}

/// Returns the weight of an undirected network in matrix (vector) form
///
/// The matrix is a vector in row major form.
/// Since the edge(i,j) = edge(j,i), I only look at the bottom left triangle
fn weight(m: &Vec<Option<usize>>, n: usize) -> usize {
    let mut total: usize = 0;
    for i in 1..n {
        for j in 0..i {
            let x = i * n + j;
            if let Some(w) = m[x] {
                total += w;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_min_weight_sample() {
        let (n, v) = sample_network();
        let w = prim(&v, n);
        assert_eq!(w, 93);
    }
    #[test]
    pub fn test_start_weight_sample() {
        let (n, v) = sample_network();
        let w = weight(&v, n);
        assert_eq!(w, 243);
    }
}
