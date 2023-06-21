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
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let s = s.into_bytes();
        let mut set = HashSet::new();
        for s in dictionary {
            set.insert(s.into_bytes());
        }
        let mut dp = vec![-1; s.len() + 1];
        dp[s.len()] = 0;
        Self::dfs(0, &s, &set, &mut dp);
        dp[0]
    }

    fn dfs(idx: usize, s: &Vec<u8>, set: &HashSet<Vec<u8>>, dp: &mut Vec<i32>) {
        if dp[idx] != -1 {
            return;
        }
        let mut g = s.len() as i32;
        for i in idx + 1..=s.len() {
            Self::dfs(i, s, set, dp);
            if set.contains(&s[idx..i]) {
                g = g.min(dp[i])
            } else {
                g = g.min(dp[i] + (i - idx) as i32);
            }
        }
        dp[idx] = g;
    }
}
