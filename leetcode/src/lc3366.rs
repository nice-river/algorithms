#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 8, 3, 19, 3];
        let k = 3;
        let op1 = 1;
        let op2 = 1;
        let ans = 23;
        assert_eq!(Solution::min_array_sum(nums, k, op1, op2), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![2, 4, 3];
        let k = 3;
        let op1 = 2;
        let op2 = 1;
        let ans = 3;
        assert_eq!(Solution::min_array_sum(nums, k, op1, op2), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![12, 15];
        let k = 11;
        let op1 = 2;
        let op2 = 1;
        let ans = 8;
        assert_eq!(Solution::min_array_sum(nums, k, op1, op2), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 3, 5, 7, 9, 12, 12, 12, 13, 15, 15, 15, 16, 17, 19, 20];
        let k = 11;
        let op1 = 15;
        let op2 = 4;
        let ans = 77;
        assert_eq!(Solution::min_array_sum(nums, k, op1, op2), ans);
    }

    #[test]
    fn test4() {
        let nums = vec![10];
        let k = 3;
        let op1 = 1;
        let op2 = 1;
        let ans = 2;
        assert_eq!(Solution::min_array_sum(nums, k, op1, op2), ans);
    }

    #[test]
    fn test5() {
        let nums = vec![10, 6];
        let k = 10;
        let op1 = 0;
        let op2 = 1;
        let ans = 6;
        assert_eq!(Solution::min_array_sum(nums, k, op1, op2), ans);
    }

    #[test]
    fn test6() {
        let nums = vec![6, 8, 3];
        let k = 8;
        let op1 = 1;
        let op2 = 3;
        let ans = 6;
        assert_eq!(Solution::min_array_sum(nums, k, op1, op2), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn min_array_sum(nums: Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
        let n = nums.len();
        let op1 = op1 as usize;
        let op2 = op2 as usize;
        let mut dp = vec![vec![vec![i32::MAX; op2 + 1]; op1 + 1]; n + 1];
        for x in 0..=op1 {
            for y in 0..=op2 {
                dp[0][x][y] = 0;
            }
        }
        for i in 0..n {
            dp[i + 1][0][0] = dp[i][0][0] + nums[i];
        }
        for j in 0..op1 {
            for i in 0..n {
                dp[i + 1][j + 1][0] = dp[i + 1][j + 1][0].min(dp[i][j + 1][0] + nums[i]);
                dp[i + 1][j + 1][0] = dp[i + 1][j + 1][0].min(dp[i][j][0] + ((nums[i] + 1) / 2));
            }
        }
        for j in 0..op2 {
            for i in 0..n {
                dp[i + 1][0][j + 1] = dp[i + 1][0][j + 1].min(dp[i][0][j + 1] + nums[i]);
                if nums[i] >= k {
                    dp[i + 1][0][j + 1] = dp[i + 1][0][j + 1].min(dp[i][0][j] + (nums[i] - k));
                }
            }
        }
        for x in 1..=n {
            for y in 1..=op1 {
                for z in 1..=op2 {
                    dp[x][y][z] = dp[x - 1][y][z] + nums[x - 1];
                    dp[x][y][z] = dp[x][y][z].min(dp[x - 1][y - 1][z] + (nums[x - 1] + 1) / 2);
                    if nums[x - 1] >= k {
                        dp[x][y][z] = dp[x][y][z].min(dp[x - 1][y][z - 1] + nums[x - 1] - k);
                    }
                    let g = if (nums[x - 1] + 1) / 2 >= k {
                        (nums[x - 1] + 1) / 2 - k
                    } else if nums[x - 1] >= k {
                        (nums[x - 1] - k + 1) / 2
                    } else {
                        (nums[x - 1] + 1) / 2
                    };
                    dp[x][y][z] = dp[x][y][z].min(dp[x - 1][y - 1][z - 1] + g);
                }
            }
        }

        dp[n][op1][op2]
    }
}
