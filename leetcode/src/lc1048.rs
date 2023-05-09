#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test0() {
        let words = vec!["a", "b", "ba", "bca", "bda", "bdca"];
        let words = words.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();
        assert_eq!(Solution::longest_str_chain(words), 4);
    }
}

struct Solution {}

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words
            .into_iter()
            .map(|s| s.into_bytes())
            .collect::<Vec<_>>();
        words.sort_by_key(|x| x.len());
        let mut dp = vec![0; words.len()];
        for i in 0..words.len() {
            Self::dfs(&words, i, &mut dp);
        }
        dp.into_iter().max().unwrap()
    }

    fn dfs(words: &Vec<Vec<u8>>, idx: usize, dp: &mut Vec<i32>) -> i32 {
        if dp[idx] != 0 {
            return dp[idx];
        }
        let mut c = 0;
        for j in idx + 1..words.len() {
            if words[j].len() == words[idx].len() + 1 {
                if Self::check(&words[idx], &words[j]) {
                    c = c.max(Self::dfs(words, j, dp));
                }
            }
        }
        dp[idx] = 1 + c;
        1 + c
    }

    fn check(a: &[u8], b: &[u8]) -> bool {
        let mut i = 0;
        let mut j = 0;
        let mut skip = false;
        while i < a.len() && j < b.len() {
            if a[i] == b[j] {
                i += 1;
                j += 1;
            } else if !skip {
                j += 1;
                skip = true;
            } else {
                return false;
            }
        }
        true
    }
}
