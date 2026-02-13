#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums1 = vec![1, 3, 2];
        let nums2 = vec![4, 5, 1];
        let k = 2;
        let ans = 22;
        assert_eq!(Solution::max_score(nums1, nums2, k), ans);
    }

    #[test]
    fn test1() {
        let nums1 = vec![-2, 0, 5];
        let nums2 = vec![-3, 4, -1, 2];
        let k = 2;
        let ans = 26;
        assert_eq!(Solution::max_score(nums1, nums2, k), ans);
    }

    #[test]
    fn test2() {
        let nums1 = vec![-3, -2];
        let nums2 = vec![1, 2];
        let k = 2;
        let ans = -7;
        assert_eq!(Solution::max_score(nums1, nums2, k), ans);
    }

    #[test]
    fn test3() {
        let nums1 = vec![-3];
        let nums2 = vec![0, -1];
        let k = 1;
        let ans = 3;
        assert_eq!(Solution::max_score(nums1, nums2, k), ans);
    }
}

use std::vec;

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let n = nums1.len();
        let m = nums2.len();
        let k = k as usize;
        let mut dp = vec![vec![vec![i64::MIN; m + 1]; n + 1]; k + 1];
        for i in 0..n + 1 {
            for j in 0..m + 1 {
                dp[0][i][j] = 0;
            }
        }
        for i in 1..n + 1 {
            for j in 1..m + 1 {
                dp[1][i][j] = dp[0][i - 1][j - 1] + nums1[i - 1] as i64 * nums2[j - 1] as i64;
                dp[1][i][j] = dp[1][i][j].max(dp[1][i - 1][j]);
                dp[1][i][j] = dp[1][i][j].max(dp[1][i][j - 1]);
            }
        }
        for x in 2..k + 1 {
            for i in x..n + 1 {
                for j in x..m + 1 {
                    dp[x][i][j] =
                        dp[x - 1][i - 1][j - 1] + nums1[i - 1] as i64 * nums2[j - 1] as i64;
                    dp[x][i][j] = dp[x][i][j].max(dp[x][i - 1][j]);
                    dp[x][i][j] = dp[x][i][j].max(dp[x][i][j - 1]);
                }
            }
        }

        dp[k][n][m]
    }
}
