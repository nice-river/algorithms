#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let word = "aabb";
        let ans = 9;
        assert_eq!(Solution::wonderful_substrings(word.to_owned()), ans);
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
    pub fn wonderful_substrings(word: String) -> i64 {
        let word = word.as_bytes();
        let mut ans = 0;
        let mut s = 0i32;
        let mut map = HashMap::new();
        for i in 0..word.len() {
            let c = (word[i] - b'a') as i32;
            s ^= 1 << c;
            if let Some(t) = map.get(&s) {
                ans += t;
            }
            for i in 0..=10 {
                let k = s ^ (1 << i);
                if let Some(t) = map.get(&k) {
                    ans += t;
                }
            }
            if s.count_ones() == 1 || s == 0 {
                ans += 1;
            }
            *map.entry(s).or_insert(0) += 1;
        }
        ans
    }
}
