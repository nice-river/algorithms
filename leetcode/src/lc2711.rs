#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                let mut a = HashSet::new();
                let mut b = HashSet::new();
                let mut k = 1;
                while i >= k && j >= k {
                    a.insert(grid[i - k][j - k]);
                    k += 1;
                }
                k = 1;
                while i + k < n && j + k < m {
                    b.insert(grid[i + k][j + k]);
                    k += 1;
                }
                ans[i][j] = (a.len() as i32 - b.len() as i32).abs();
            }
        }
        ans
    }
}
