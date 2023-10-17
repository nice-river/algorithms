#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

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

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut set = HashSet::new();
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for word in &word_dict {
            set.insert(word.as_bytes());
        }
        for i in 1..=n {
            for j in 0..i {
                if set.contains(&s[j..i]) && dp[j] {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}
