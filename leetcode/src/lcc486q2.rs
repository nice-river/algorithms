#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![5, 4, -9, 6];
        let k = 2;
        let ans = vec![6, 5, -9, 4];
        assert_eq!(Solution::rotate_elements(nums, k), ans);
    }
}

use std::{iter::Cloned, task::Poll};

use crate::*;

struct Solution {}

impl Solution {
    pub fn rotate_elements(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return nums;
        }
        let positivies = nums
            .iter()
            .filter(|x| !x.is_negative())
            .copied()
            .collect::<Vec<i32>>();
        if positivies.is_empty() {
            return nums;
        }
        let k = k as usize;
        let mut idx = k % positivies.len();
        for x in nums.iter_mut() {
            if !x.is_negative() {
                (*x) = positivies[idx];
                idx += 1;
                idx %= positivies.len();
            }
        }
        nums
    }
}
