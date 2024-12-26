#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 2, 2, 3, 3, 4];
        let k = 2;
        let ans = 6;
        assert_eq!(Solution::max_distinct_elements(nums, k), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        nums.sort();
        let mut ans = 1;
        let mut mini = nums[0] - k;
        for num in nums.into_iter().skip(1) {
            if num - k > mini {
                mini = num - k;
                ans += 1;
            } else if mini + 1 <= num + k {
                mini += 1;
                ans += 1;
            }
        }
        ans
    }
}
