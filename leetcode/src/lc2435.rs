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

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let t = k as usize;
        let mut dp = vec![vec![vec![0; t]; m + 1]; n + 1];
        dp[1][1][grid[0][0] as usize % t] = 1;
        let module = 1000000007i64;
        for i in 1..=n {
            for j in 1..=m {
                for k in 0..t {
                    let x = grid[i - 1][j - 1] as usize % t;
                    if i - 1 >= 1 {
                        dp[i][j][(k + x) % t] += dp[i - 1][j][k];
                        dp[i][j][(k + x) % t] %= module;
                    }
                    if j - 1 >= 1 {
                        dp[i][j][(k + x) % t] += dp[i][j - 1][k];
                        dp[i][j][(k + x) % t] %= module;
                    }
                }
            }
        }
        dp[n][m][0] as i32
    }
}
