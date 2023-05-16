#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![-1; m]; n];
        for i in 0..n {
            dp[i][0] = 0;
        }
        for j in 1..m {
            for i in 0..n {
                if grid[i][j] > grid[i][j - 1] && dp[i][j - 1] != -1 {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1] + 1);
                }
                if i > 0 && grid[i][j] > grid[i - 1][j - 1] && dp[i - 1][j - 1] != -1 {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
                }
                if i < n - 1 && grid[i][j] > grid[i + 1][j - 1] && dp[i + 1][j - 1] != -1 {
                    dp[i][j] = dp[i][j].max(dp[i + 1][j - 1] + 1);
                }
            }
        }

        dp.into_iter()
            .map(|k| k.into_iter().max().unwrap())
            .max()
            .unwrap()
    }
}
