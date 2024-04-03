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

use std::collections::BTreeSet;

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let s = s.as_bytes();
        let mut set = BTreeSet::new();
        for i in 0..s.len() - 1 {
            let t = &s[i..i + 2];
            set.insert([s[i], s[i + 1]]);
        }
        for i in 0..s.len() - 1 {
            let t = [s[i + 1], s[i]];
            if set.contains(&t) {
                return true;
            }
        }
        false
    }
}
