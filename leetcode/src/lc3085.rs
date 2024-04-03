#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let word = "dabdcbdcdcd".to_owned();
        let k = 2;
        let ans = 2;
        assert_eq!(Solution::minimum_deletions(word, k), ans);
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
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut map = HashMap::new();
        for b in word.as_bytes() {
            *map.entry(b).or_insert(0) += 1;
        }
        let mut cnts = map.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
        cnts.sort();
        let mut ans = i32::MAX;
        for i in 0..cnts.len() {
            let mut t = 0;
            for j in 0..i {
                t += cnts[j];
            }
            for j in i + 1..cnts.len() {
                if cnts[j] - cnts[i] > k {
                    t += cnts[j] - (cnts[i] + k);
                }
            }
            ans = ans.min(t);
        }
        for i in 0..cnts.len() {
            let mut t = 0;
            for j in 0..i {
                if cnts[j] < cnts[i] - k {
                    t += cnts[j];
                }
            }
            for j in i + 1..cnts.len() {
                if cnts[j] > cnts[i] {
                    t += cnts[j];
                }
            }
            ans = ans.min(t);
        }
        ans
    }
}
