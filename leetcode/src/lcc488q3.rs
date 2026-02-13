#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 3, 2];
        let k = 4;
        let ans = 5;
        assert_eq!(Solution::count_subarrays(nums, k), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![5, 5, 5, 5];
        let k = 0;
        let ans = 10;
        assert_eq!(Solution::count_subarrays(nums, k), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3];
        let k = 0;
        let ans = 3;
        assert_eq!(Solution::count_subarrays(nums, k), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![13];
        let k = 820;
        let ans = 1;
        assert_eq!(Solution::count_subarrays(nums, k), ans);
    }
}

use crate::*;

struct Solution {}

use std::cmp::Reverse;
use std::collections::BTreeSet;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        if nums.is_empty() {
            return 0;
        }
        let mut max_set = BTreeSet::new();
        let mut min_set = BTreeSet::new();
        let mut i = 0;
        let mut j = 0;
        let mut ans = 0;
        let k = k as usize;
        while j < nums.len() {
            if i == j {
                max_set.insert(Reverse((nums[j] as usize, j)));
                min_set.insert((nums[j] as usize, j));
                j += 1;
                continue;
            }
            let max = max_set.iter().next().unwrap().0 .0;
            let min = min_set.iter().next().unwrap().0;
            if (max - min) * (j - i) <= k {
                let l = (j - i) as i64;
                ans += l;
                max_set.insert(Reverse((nums[j] as usize, j)));
                min_set.insert((nums[j] as usize, j));
                j += 1;
            } else {
                max_set.remove(&Reverse((nums[i] as usize, i)));
                min_set.remove(&(nums[i] as usize, i));
                i += 1;
            }
        }
        if i < nums.len() {
            loop {
                let max = max_set.iter().next().unwrap().0 .0;
                let min = min_set.iter().next().unwrap().0;
                if (max - min) * (j - i) <= k {
                    break;
                } else {
                    max_set.remove(&Reverse((nums[i] as usize, i)));
                    min_set.remove(&(nums[i] as usize, i));
                    i += 1;
                }
            }
        }
        let l = (j - i) as i64;
        ans += l;
        ans
    }
}
