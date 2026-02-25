#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 3, 2];
        let k = 6;
        let ans = 2;
        assert_eq!(Solution::count_sequences(nums, k), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![4, 6, 3];
        let k = 2;
        let ans = 2;
        assert_eq!(Solution::count_sequences(nums, k), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 5];
        let k = 1;
        let ans = 3;
        assert_eq!(Solution::count_sequences(nums, k), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 1];
        let k = 1;
        let ans = 9;
        assert_eq!(Solution::count_sequences(nums, k), ans);
    }
}

use crate::*;

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn count_sequences(nums: Vec<i32>, k: i64) -> i32 {
        let mut memo = HashMap::new();
        Self::dfs(1, 1, 0, &nums, &mut memo, k)
    }

    fn dfs(
        d: i64,
        n: i64,
        p: usize,
        nums: &Vec<i32>,
        memo: &mut HashMap<(i64, i64, usize), i32>,
        k: i64,
    ) -> i32 {
        if p == nums.len() {
            if n * k == d {
                return 1;
            }
            return 0;
        }
        if let Some(&res) = memo.get(&(d, n, p)) {
            return res;
        }
        let mut res = 0;
        let num = nums[p] as i64;
        res += Self::dfs(d, n, p + 1, nums, memo, k);
        let g = Self::gcd(d * num, n);
        res += Self::dfs(d * num / g, n / g, p + 1, nums, memo, k);
        let g = Self::gcd(d, n * num);
        res += Self::dfs(d / g, n * num / g, p + 1, nums, memo, k);
        memo.insert((d, n, p), res);
        res
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
