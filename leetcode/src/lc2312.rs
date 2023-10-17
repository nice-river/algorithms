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
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for price in prices {
            let h = price[0];
            let w = price[1];
            let p = price[2] as i64;
            dp[h as usize][w as usize] = p;
        }
        for i in 1..=m {
            for j in 1..=n {
                for k in 1..=i / 2 {
                    dp[i][j] = dp[i][j].max(dp[k][j] + dp[i - k][j]);
                }
                for k in 1..=j / 2 {
                    dp[i][j] = dp[i][j].max(dp[i][k] + dp[i][j - k]);
                }
            }
        }
        dp[m][n]
    }
}
