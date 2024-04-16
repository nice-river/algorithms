#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "aaxxx".to_owned();
        let k = 1;
        let ans = 4;
        assert_eq!(Solution::max_partitions_after_operations(s, k), ans);
    }

    #[test]
    fn test1() {
        let s = "baac".to_owned();
        let k = 3;
        let ans = 2;
        assert_eq!(Solution::max_partitions_after_operations(s, k), ans);
    }

    #[test]
    fn test2() {
        let s = "ba".to_owned();
        let k = 1;
        let ans = 2;
        assert_eq!(Solution::max_partitions_after_operations(s, k), ans);
    }

    #[test]
    fn test3() {
        let s = "hnhdfs".to_owned();
        let k = 5;
        let ans = 2;
        assert_eq!(Solution::max_partitions_after_operations(s, k), ans);
    }

    #[test]
    fn test4() {
        let s = "yrfajcymwdlzrrmheca".to_owned();
        let k = 6;
        let ans = 3;
        assert_eq!(Solution::max_partitions_after_operations(s, k), ans);
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
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        if k == 26 {
            return 1;
        }
        let s = s.as_bytes();
        let mut dp = HashMap::new();
        Self::dfs(0, 0, false, s, k as u32, &mut dp)
    }

    fn dfs(idx: usize, mask: i32, changed: bool, s: &[u8], k: u32, dp: &mut HashMap<(usize, i32, bool), i32>) -> i32 {
        if idx == s.len() {
            return 1;
        }
        if let Some(x) = dp.get(&(idx, mask, changed)) {
            return *x;
        }
        let bit = 1 << (s[idx] - b'a');
        let new_mask = mask | bit;
        let mut res;
        if new_mask.count_ones() > k {
            res = Self::dfs(idx + 1, bit, changed, s, k, dp) + 1;
        } else {
            res = Self::dfs(idx + 1, new_mask, changed, s, k, dp);
        }
        if !changed {
            for c in 0..26 {
                let new_mask = mask | (1 << c);
                if new_mask.count_ones() > k {
                    res = res.max(Self::dfs(idx + 1, 1 << c, true, s, k, dp) + 1);
                } else {
                    res = res.max(Self::dfs(idx + 1, new_mask, true, s, k, dp));
                }
            }
        }
        dp.insert((idx, mask, changed), res);
        res
    }
}