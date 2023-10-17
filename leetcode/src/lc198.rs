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
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0, 0]; n];
        dp[0][1] = nums[0];
        for i in 1..n {
            dp[i][0] = dp[i - 1][0].max(dp[i - 1][1]);
            dp[i][1] = dp[i - 1][0] + nums[i];
        }
        dp[n - 1][0].max(dp[n - 1][1])
    }
}
