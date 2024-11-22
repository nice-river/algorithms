#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 4, 5];
        let k = 1;
        let num_operations = 2;
        let ans = 2;
        assert_eq!(Solution::max_frequency(nums, k, num_operations), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![5, 11, 20, 20];
        let k = 5;
        let num_operations = 1;
        let ans = 2;
        assert_eq!(Solution::max_frequency(nums, k, num_operations), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 2];
        let k = 1;
        let num_operations = 1;
        let ans = 3;
        assert_eq!(Solution::max_frequency(nums, k, num_operations), ans);
    }
}

use crate::*;

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut not_used = nums.len() as i32;
        let num_operations = not_used - num_operations;
        let mut arr = vec![];
        for num in nums {
            arr.push((num as i64 - k as i64, -1));
            arr.push((num as i64, 0));
            arr.push((num as i64 + k as i64, 1));
        }
        arr.sort();
        let mut cnt = 0;
        let mut ans = 0;
	let mut freq = HashMap::new();
        for (axis, lr) in arr {
            match lr {
                -1 => {
                    not_used -= 1;
                    cnt += 1;
                    ans = ans.max(cnt - 0.max(num_operations - not_used));
                }
		0 => {
		    let entry = freq.entry(axis).or_insert(0);
		    *entry += 1;
		    let f = *entry;
		    let remv = 0.max(num_operations - not_used - f);
		    ans = ans.max(cnt - remv);
		}
                1 => {
                    not_used += 1;
                    cnt -= 1;
                }
                _ => unreachable!(),
            }
        }
        ans
    }
}
