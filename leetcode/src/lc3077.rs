#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![-1000, 1, 2, 3, -1, 2];
        let k = 3;
        let ans = 22;
        assert_eq!(Solution::maximum_strength(nums, k), ans);

        let nums = vec![12, -2, -2, -2, -2];
        let k = 5;
        let ans = 64;
        assert_eq!(Solution::maximum_strength(nums, k), ans);

        let nums = vec![-2, -1, -3];
        let k = 1;
        let ans = -1;
        assert_eq!(Solution::maximum_strength(nums, k), ans);

        let nums = vec![-2, -1, -3];
        let k = 3;
        let ans = -7;
        assert_eq!(Solution::maximum_strength(nums, k), ans);
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
    pub fn maximum_strength(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut dp = vec![vec![vec![i64::MIN / 2; k + 1]; n + 1]; 2];
        dp[1][0][0] = 0;
        for i in 1..=n {
            for j in 0..=k {
                dp[0][i][j] = dp[0][i - 1][j].max(dp[1][i - 1][j]);
                if j > 0 {
                    dp[1][i][j] = dp[1][i - 1][j] + Self::weight(nums[i - 1], k - j + 1);
                    dp[1][i][j] = dp[1][i][j].max(
                        dp[1][i - 1][j - 1].max(dp[0][i - 1][j - 1])
                            + Self::weight(nums[i - 1], k - j + 1),
                    );
                }
            }
        }
        dp[0][n][k].max(dp[1][n][k])
    }

    fn weight(v: i32, k: usize) -> i64 {
        v as i64 * k as i64 * if k % 2 == 0 { -1 } else { 1 }
    }
}
