#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![4, 3, 5, 4];
        let ans = 2;
        assert_eq!(Solution::median_of_uniqueness_array(nums), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![99, 73, 14, 84, 14, 73, 14, 14, 14];
        let ans = 2;
        assert_eq!(Solution::median_of_uniqueness_array(nums), ans);
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

use std::collections::HashMap;

impl Solution {
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let n = (nums.len() + 1) * nums.len() / 2;
        let mid_idx = (n - 1) / 2 + 1;
        let mut l = 1;
        let mut r = nums.len();
        while l < r {
            let m = (l + r) / 2;
            let mut cnt = 0;
            let mut map = HashMap::new();
            let mut g = 0;
            for i in 0..nums.len() {
                *map.entry(nums[i]).or_insert(0) += 1;
                while map.len() > m {
                    *map.get_mut(&nums[g]).unwrap() -= 1;
                    if map.get(&nums[g]).unwrap() == &0 {
                        map.remove(&nums[g]);
                    }
                    g += 1;
                }
                cnt += i + 1 - g;
            }
            if cnt >= mid_idx {
                r = m;
            } else {
                l = m + 1;
            }
        }
        r as i32
    }
}
