#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![8, 7, 6, 6];
        let ans = 3;
        assert_eq!(Solution::incremovable_subarray_count(nums), ans);
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
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let n = nums.len();
        let mut j = n - 1;
        while j > 0 && nums[j - 1] < nums[j] {
            j = j - 1;
        }
        ans += (n - j) as i64 + 1;
        if j == 0 {
            ans -= 1;
        }
        for i in 1..n {
            j = j.max(i + 1);
            while j < n && nums[j] <= nums[i - 1] {
                j += 1;
            }
            ans += (n - j) as i64 + 1;
            if nums[i] <= nums[i - 1] {
                break;
            }
        }
        ans
    }
}