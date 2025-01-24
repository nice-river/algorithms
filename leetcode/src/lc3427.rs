#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 3, 1];
        let ans = 11;
        assert_eq!(Solution::subarray_sum(nums), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            let start = 0.max(i as i32 - nums[i]) as usize;
            for j in start..=i {
                ans += nums[j];
            }
        }
        ans
    }
}
