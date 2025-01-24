#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 2, 3];
        let k = 2;
        let ans = 24;
        assert_eq!(Solution::min_max_sums(nums, k), ans);

        let nums = vec![5, 0, 6];
        let k = 1;
        let ans = 22;
        assert_eq!(Solution::min_max_sums(nums, k), ans);

        let nums = vec![1, 1, 1];
        let k = 2;
        let ans = 12;
        assert_eq!(Solution::min_max_sums(nums, k), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn min_max_sums(mut nums: Vec<i32>, k: i32) -> i32 {
        const MODULE: i64 = 1e9 as i64 + 7;
        let mut ans = 0i64;
        nums.sort();
        let mut cnts = vec![0; k as usize];
        cnts[0] = 1;
        for i in 0..nums.len() {
            for j in 0..k as usize {
                ans += nums[i] as i64 * cnts[j] % MODULE;
                ans %= MODULE;
                ans += nums[nums.len() - 1 - i] as i64 * cnts[j] % MODULE;
                ans %= MODULE;
            }
            for j in (1..k as usize).rev() {
                cnts[j] += cnts[j - 1];
                cnts[j] %= MODULE;
            }
        }
        ans as i32
    }
}