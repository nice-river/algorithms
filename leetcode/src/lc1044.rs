#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "banana";
        let ans = "ana";
        assert_eq!(Solution::longest_dup_substring(s.into()), ans);
    }

    #[test]
    fn test1() {
        let s = "aa";
        let ans = "a";
        assert_eq!(Solution::longest_dup_substring(s.into()), ans);
    }

    #[test]
    fn test2() {
        let s = "aaaaaaaaaaaaaaaaaaaaaa";
        let ans = "aaaaaaaaaaaaaaaaaaaaa";
        assert_eq!(Solution::longest_dup_substring(s.into()), ans);

        let s = "ababababababababababab";
        let ans = "abababababababababab";
        assert_eq!(Solution::longest_dup_substring(s.into()), ans);

        let s = "efgabcdabcdabcefgab";
        let ans = "abcdabc";
        assert_eq!(Solution::longest_dup_substring(s.into()), ans);
    }
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let s = s.as_bytes();
        let primes = vec![(10i64).pow(9) + 7, 301369891];
        let mut substr_mods = vec![vec![0; primes.len()]; s.len() + 1];
        let mut base_mods = vec![vec![1; primes.len()]; s.len() + 1];
        for i in 0..s.len() {
            for (j, &p) in primes.iter().enumerate() {
                base_mods[i + 1][j] = base_mods[i][j] * 26 % p;
                substr_mods[i + 1][j] =
                    (substr_mods[i][j] + (s[i] - b'a') as i64 * base_mods[i + 1][j]) % p;
            }
        }
        let mut l = 1;
        let mut r = s.len() - 1;
        let mut set = HashSet::new();
        let mut ans = s;

        while l < r || (l == r && l != 0) {
            set.clear();
            let m = (l + r) / 2;
            let mut found_idx = None;
            for i in 0..s.len() - m + 1 {
                let mut t = Vec::with_capacity(primes.len());
                for (j, &p) in primes.iter().enumerate() {
                    let k = (substr_mods[i + m][j] - substr_mods[i][j] + p) % p;
                    let k = (k * base_mods[s.len() + 1 - (i + m)][j]) % p;
                    t.push(k);
                }
                if set.contains(&t) {
                    found_idx = Some(i);
                    break;
                }
                set.insert(t);
            }
            if let Some(idx) = found_idx {
                ans = &s[idx..idx + m];
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        if ans.len() == s.len() {
            "".into()
        } else {
            std::str::from_utf8(ans).unwrap_or("").into()
        }
    }
}
