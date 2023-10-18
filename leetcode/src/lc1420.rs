#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 4;
        let m = 3;
        let k = 2;
        let ans = 39;
        assert_eq!(Solution::num_of_arrays(n, m, k), ans);
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

struct Solution {}

impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        const MODULE: i64 = 1_000_000_007;
        let n = n as usize;
        let m = m as usize;
        let k = k as usize;
        let mut dp = vec![vec![0; m + 1]; k + 1];
        dp[0][0] = 1;
        for _ in 0..n {
            let mut ndp = vec![vec![0; m + 1]; k + 1];
            for i in 1..=k {
                let mut sum = dp[i - 1][0];
                for j in 1..=m {
                    ndp[i][j] = dp[i][j] * j as i64;
                    ndp[i][j] %= MODULE;
                    ndp[i][j] += sum;
                    ndp[i][j] %= MODULE;
                    sum += dp[i - 1][j];
                }
            }
            dp = ndp;
        }
        let mut ans = 0;
        for i in 1..=m {
            ans += dp[k][i];
            ans %= MODULE;
        }
        ans as i32
    }
}
