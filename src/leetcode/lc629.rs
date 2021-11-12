#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 3;
        let k = 0;
        let ans = 1;
        assert_eq!(Solution::k_inverse_pairs(n, k), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let module = 10i64.pow(9) + 7;
        let mut dp = vec![vec![0; k + 1]; n + 1];
        dp[1][0] = 1;
        for i in 1..=n {
            dp[i][0] = 1;
            for j in 1..=k {
                if j <= i - 1 {
                    dp[i][j] = (dp[i][j - 1] + dp[i - 1][j]) % module;
                } else {
                    let k =
                        ((dp[i - 1][j] - dp[i - 1][j - (i - 1) - 1]) % module + module) % module;
                    dp[i][j] = (dp[i][j - 1] + k) % module;
                }
            }
        }
        if k == 0 {
            dp[n][k] as i32
        } else {
            (((dp[n][k] - dp[n][k - 1]) % module + module) % module) as i32
        }
    }
}
