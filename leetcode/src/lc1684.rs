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

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut set = HashSet::new();
        for ch in allowed.chars() {
            set.insert(ch);
        }
        'outer: for word in words {
            for ch in word.chars() {
                if !set.contains(&ch) {
                    continue 'outer;
                }
            }
            ans += 1;
        }
        ans
    }
}
