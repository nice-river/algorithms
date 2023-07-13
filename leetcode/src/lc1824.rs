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
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let n = obstacles.len();
        let mut dp = vec![vec![i32::MAX; 3]; n];
        dp[0][1] = 0;
        dp[0][0] = 1;
        dp[0][2] = 1;
        for i in 1..n {
            for j in 0..3 {
                let ob = obstacles[i] as usize;
                if j + 1 == ob {
                    dp[i][j] = i32::MAX;
                    continue;
                }
                dp[i][j] = dp[i - 1][j];
                for k in 0..3 {
                    if j == k {
                        continue;
                    }
                    if dp[i - 1][k] != i32::MAX && k + 1 != ob {
                        dp[i][j] = dp[i][j].min(dp[i - 1][k] + 1);
                    }
                }
            }
        }

        let mut ans = i32::MAX;
        for j in 0..3 {
            ans = ans.min(dp[n - 1][j]);
        }
        ans
    }
}