#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "11110100001001".to_string();
        let ans = 1;
        assert_eq!(Solution::minimum_beautiful_substrings(s), ans);
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

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn minimum_beautiful_substrings(s: String) -> i32 {
        let mut num = 0i64;
        for c in s.chars() {
            num *= 2;
            if c == '1' {
                num += 1;
            }
        }
        let mut is_beautiful = HashMap::new();
        let mut base = 1;
        while base <= num {
            is_beautiful.insert(format!("{base:b}").into_bytes(), 1);
            base *= 5;
        }
        let mut dp = HashMap::new();
        let ans = Solution::dfs(&s.into_bytes(), 0, &is_beautiful, &mut dp);
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }

    fn dfs(
        num: &Vec<u8>,
        idx: usize,
        is_beautiful: &HashMap<Vec<u8>, i32>,
        dp: &mut HashMap<usize, i32>,
    ) -> i32 {
        if idx == num.len() {
            return 0;
        }
        if dp.contains_key(&idx) {
            return *dp.get(&idx).unwrap();
        }
        let mut ret = i32::MAX;
        for i in idx + 1..=num.len() {
            if is_beautiful.contains_key(&num[idx..i]) {
                let g = Self::dfs(num, i, is_beautiful, dp);
                if g != i32::MAX {
                    ret = ret.min(g + 1);
                }
            }
        }
        dp.insert(idx, ret);
        ret
    }
}
