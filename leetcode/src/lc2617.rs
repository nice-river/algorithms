#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let grid = [[2, 1, 0], [1, 0, 0]].to_vec();
        let grid = grid.into_iter().map(|v| v.to_vec()).collect();
        let ans = -1;
        assert_eq!(Solution::minimum_visited_cells(grid), ans);
    }

    #[test]
    fn test1() {
        let grid = [[3, 4, 2, 1], [4, 2, 3, 1], [2, 1, 0, 0], [2, 4, 0, 0]].to_vec();
        let grid = grid.into_iter().map(|v| v.to_vec()).collect();
        let ans = 4;
        assert_eq!(Solution::minimum_visited_cells(grid), ans);
    }

    #[test]
    fn test2() {
        let grid = [[3, 4, 2, 1], [4, 2, 1, 1], [2, 1, 1, 0], [3, 4, 1, 0]].to_vec();
        let grid = grid.into_iter().map(|v| v.to_vec()).collect();
        let ans = 3;
        assert_eq!(Solution::minimum_visited_cells(grid), ans);
    }

    #[test]
    fn test3() {
        let grid = [[0]].to_vec();
        let grid = grid.into_iter().map(|v| v.to_vec()).collect();
        let ans = 1;
        assert_eq!(Solution::minimum_visited_cells(grid), ans);
    }

    #[test]
    fn test4() {
        let grid = [[0, 1, 0]].to_vec();
        let grid = grid.into_iter().map(|v| v.to_vec()).collect();
        let ans = -1;
        assert_eq!(Solution::minimum_visited_cells(grid), ans);
    }
}

struct Solution {}

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

impl Solution {
    pub fn minimum_visited_cells(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        if n == 1 && m == 1 {
            return 1;
        }
        let mut col_idxs = vec![BTreeSet::<(usize, usize)>::new(); m];
        let mut col_minis = vec![BTreeSet::<(i32, usize)>::new(); m];
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; m]; n];
        for i in 0..n {
            let mut row_idx = BTreeSet::new();
            let mut row_mini = BTreeSet::new();

            for j in 0..m {
                let out_of = col_idxs[j]
                    .range((Unbounded, Excluded(&(i, 0))))
                    .copied()
                    .collect::<Vec<_>>();
                for (cover, r) in out_of {
                    col_idxs[j].remove(&(cover, r));
                    col_minis[j].remove(&(dp[r][j], r));
                }

                let mut t;
                if let Some((mini, _)) = col_minis[j].iter().next() {
                    t = *mini + 1;
                } else {
                    t = i32::MAX;
                }
                let out_of = row_idx
                    .range((Unbounded, Excluded(&(j, 0))))
                    .copied()
                    .collect::<Vec<_>>();
                for (cover, c) in out_of {
                    row_idx.remove(&(cover, c));
                    row_mini.remove(&(dp[i][c], c));
                }
                if let Some((mini, _)) = row_mini.iter().next() {
                    t = t.min(*mini + 1);
                } else {
                    t = t.min(i32::MAX);
                }
                if t == i32::MAX {
                    dp[i][j] = if i == 0 && j == 0 { 1 } else { -1 };
                } else {
                    dp[i][j] = t;
                }
                if dp[i][j] != -1 {
                    col_idxs[j].insert((i + grid[i][j] as usize, i));
                    row_idx.insert((j + grid[i][j] as usize, j));

                    col_minis[j].insert((dp[i][j], i));
                    row_mini.insert((dp[i][j], j));
                }
            }
        }
        dp[n - 1][m - 1]
    }
}
