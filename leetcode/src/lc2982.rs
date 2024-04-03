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

use std::collections::{HashMap, BTreeMap};

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut set = HashMap::new();
        while i < s.len() {
            let mut j = i + 1;
            while j < s.len() && s[j] == s[i] {
                j += 1;
            }
            let e = set.entry(s[i]).or_insert_with(|| BTreeMap::<i32, i32>::new());
            *e.entry((j - i) as i32).or_insert(0) += 1;
            i = j;
        }
        let mut ans = 0;
        for (_, v) in set.into_iter() {
            for (i, (sz, cnt)) in v.into_iter().rev().enumerate() {
                if i >= 1 {
                    ans = ans.max(sz);
                } else if cnt >= 3 {
                    ans = ans.max(sz);
                } else if cnt >= 2 {
                    ans = ans.max(sz - 1);
                } else if sz >= 3 {
                    ans = ans.max(sz - 2);
                }
            }
        }
        if ans == 0 {
            -1
        } else {
            ans
        }
    }
}
