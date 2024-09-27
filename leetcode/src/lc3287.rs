#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![4, 2, 5, 6, 7];
        let k = 2;
        let ans = 2;
        assert_eq!(Solution::max_value(nums, k), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![1, 89, 11, 90];
        let k = 2;
        let ans = 2;
        assert_eq!(Solution::max_value(nums, k), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_value(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        const MAX: usize = 1 << 7;
        let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut mem = vec![vec![vec![false; MAX]; k + 1]; n];
        mem[0][1][nums[0]] = true;
        for i in 1..n {
            for j in 1..=k {
                mem[i][j] = mem[i - 1][j].clone();
            }
            mem[i][1][nums[i]] = true;
            (2..=k).for_each(|j| {
                for v in 1..MAX {
                    if mem[i - 1][j - 1][v] {
                        mem[i][j][v | nums[i]] = true;
                    }
                }
            });
        }
        let mut ans = 0;
        let mut dp = vec![vec![false; MAX]; k + 1];
        dp[0][0] = true;
        for i in (1..n).rev() {
            for j in (1..=k).rev() {
                for v in 0..MAX {
                    if dp[j - 1][v] {
                        dp[j][v | nums[i]] = true;
                    }
                }
            }
            for a in 0..MAX {
                for b in 0..MAX {
                    if mem[i - 1][k][a] && dp[k][b] {
                        ans = ans.max((a ^ b) as i32);
                    }
                }
            }
        }

        ans
    }
}
