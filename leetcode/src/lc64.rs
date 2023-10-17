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

use crate::*;

struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![i32::MAX; m + 1]; n + 1];
        dp[0][1] = 0;
        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i - 1][j - 1];
            }
        }
        dp[n][m]
    }
}
