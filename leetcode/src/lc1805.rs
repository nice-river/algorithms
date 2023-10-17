#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let word = "0a0".to_owned();
        let ans = 1;
        assert_eq!(Solution::num_different_integers(word), ans);
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
    pub fn num_different_integers(word: String) -> i32 {
        let word = word.as_bytes();
        let mut map = HashMap::new();
        let mut idx = word.len();
        for i in 0..word.len() {
            let ch = word[i];
            if ch.is_ascii_digit() {
                if idx == word.len() {
                    idx = i;
                }
            } else if idx != word.len() {
                while idx < i - 1 && word[idx] == b'0' {
                    idx += 1;
                }
                *map.entry(&word[idx..i]).or_insert(0) += 1;
                idx = word.len();
            }
        }
        if idx != word.len() {
            while idx < word.len() - 1 && word[idx] == b'0' {
                idx += 1;
            }
            *map.entry(&word[idx..word.len()]).or_insert(0) += 1;
        }
        map.len() as i32
    }
}