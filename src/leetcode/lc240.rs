#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        let m = matrix.first().unwrap().len();
        let mut i = 0;
        let mut j = m - 1;
        loop {
            match matrix[i][j].cmp(&target) {
                Ordering::Less => {
                    if i == n - 1 {
                        return false;
                    }
                    i += 1;
                }
                Ordering::Greater => {
                    if j == 0 {
                        return false;
                    }
                    j -= 1;
                }
                Ordering::Equal => {
                    return true;
                }
            }
        }
    }
}
