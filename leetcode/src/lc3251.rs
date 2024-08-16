#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 3, 2];
        let ans = 4;
        assert_eq!(Solution::count_of_pairs(nums), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![5, 5, 5, 5];
        let ans = 126;
        assert_eq!(Solution::count_of_pairs(nums), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        const MODULE: i64 = 1_000_000_007;
        let n = (*nums.iter().max().unwrap()) as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=nums[0] as usize {
            dp[i] = dp[i - 1] + 1;
        }
        for i in 1..nums.len() {
            let mut mem = vec![0; n + 1];
            for j in 0..=nums[i] as usize {
                let t = (nums[i - 1] - nums[i] + j as i32).min(j as i32);
                if t < 0 {
                    continue;
                }
                mem[j] = dp[t as usize];
            }
	    dp[..].copy_from_slice(&mem);
            for i in 1..dp.len() {
                dp[i] += dp[i - 1];
                dp[i] %= MODULE;
            }
        }
	*dp.last().unwrap() as i32
    }
}
