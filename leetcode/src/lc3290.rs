#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let a = vec![3, 2, 5, 6];
        let b = vec![2, -6, 4, -5, -3, 2, -7];
        let ans = 26;
        assert_eq!(Solution::max_score(a, b), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let mut dp = vec![0; b.len()];
        dp[0] = a[0] as i64 * b[0] as i64;
        for j in 1..b.len() {
            dp[j] = dp[j - 1].max(a[0] as i64 * b[j] as i64);
        }
        for i in 1..a.len() {
            let mut new_dp = dp.clone();
            new_dp[i] = new_dp[i - 1] + a[i] as i64 * b[i] as i64;
            for j in i + 1..b.len() {
                new_dp[j] = new_dp[j - 1].max(a[i] as i64 * b[j] as i64 + dp[j - 1]);
            }
            dp = new_dp;
        }
        *dp.last().unwrap()
    }
}
