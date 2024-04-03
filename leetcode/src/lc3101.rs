#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 0, 1, 0];
        let ans = 10;
        assert_eq!(Solution::count_alternating_subarrays(nums), ans);

        let nums = vec![0, 1, 1, 1];
        let ans = 5;
        assert_eq!(Solution::count_alternating_subarrays(nums), ans);
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
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let mut ans = 1;
        let mut p = 1;
        for i in 1..nums.len() {
            ans += 1;
            if nums[i] != nums[i - 1] {
                ans += p;
                p += 1;
            } else {
                p = 1;
            }
        }
        ans
    }
}
