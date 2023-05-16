use std::{num, vec};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![0, 0];
        let ans = vec![1, 1];
        assert_eq!(Solution::distance(nums), ans);
    }
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut ans = vec![0; n];
        let mut map = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            map.entry(num).or_insert(vec![]).push(i);
        }
        for (_, pos) in map.iter() {
            let mut s = pos.iter().sum::<usize>();
            let mut k = pos.len();
            let mut ls = 0;
            let mut lc = 0;
            for &p in pos {
                let t = s - p - (k - 1) * p + p * lc - ls;
                k -= 1;
                lc += 1;
                ls += p;
                s -= p;
                ans[p] = t as i64;
            }
        }
        ans
    }
}
