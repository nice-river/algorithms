#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let ans = 4;
        assert_eq!(Solution::search(nums, target), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![7, 8, 1, 2, 3, 4, 5, 6];
        let target = 2;
        let ans = 3;
        assert_eq!(Solution::search(nums, target), ans);
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
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = (left + right) / 2;
            if target == nums[mid] {
                return mid as i32;
            } else if target < nums[mid] {
                if nums[mid] < nums[left] {
                    right = mid;
                } else if target >= nums[left] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid] > nums[left] {
                    left = mid + 1;
                } else if target < nums[left] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
        }
        -1
    }
}
