#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let grid = to_vec2d([[1, 2]]);
        let ans = 1;
        assert_eq!(Solution::max_score(grid), ans);
    }

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

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![i32::MIN; m]; n];
        let mut ans = i32::MIN;
        for i in 0..n {
            for j in 0..m {
                if i > 0 {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j].max(0) + grid[i][j] - grid[i - 1][j]);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1].max(0) + grid[i][j] - grid[i][j - 1]);
                }
                ans = ans.max(dp[i][j]);
            }
        }
        ans
    }
}
