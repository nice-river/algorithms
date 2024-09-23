#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let words = vec!["ab".to_owned()];
        let target = "a".to_owned();
        let ans = 1;
        assert_eq!(Solution::min_valid_strings(words, target), ans);
    }

    #[test]
    fn test1() {
        let words = vec!["abc", "aaaaa", "bcdef"];
        let target = "aabcdabc".to_owned();
        //let target = "abc".to_owned();
        let ans = 3;
        assert_eq!(
            Solution::min_valid_strings(words.into_iter().map(|s| s.to_owned()).collect(), target),
            ans
        );
    }

    #[test]
    fn test2() {
        let words = vec!["abababab", "ab"];
        let target = "ababaababa".to_owned();
        let ans = 2;
        assert_eq!(
            Solution::min_valid_strings(words.into_iter().map(|s| s.to_owned()).collect(), target),
            ans
        );
    }

    #[test]
    fn test3() {
        let words = vec!["abcdef"];
        let target = "xyz".to_owned();
        let ans = -1;
        assert_eq!(
            Solution::min_valid_strings(words.into_iter().map(|s| s.to_owned()).collect(), target),
            ans
        );
    }

    #[test]
    fn test4() {
        let words = vec!["abab", "ababc", "ababa"];
        let target = "abbababc".to_owned();
        let ans = -1;
        assert_eq!(
            Solution::min_valid_strings(words.into_iter().map(|s| s.to_owned()).collect(), target),
            ans
        );
    }

    #[test]
    fn test5() {
        let words = vec!["bc","acacccaacabbaaac","cbbbcb","c","aabaaaccacbbbcacbbbbabcbccbcbaabacacbaabbcbbcabbcbcbacaacbbaabaaccacabbbbabbbbbbabacbccacbcbccbcbbabbabcabaaaccbababcccccccaaacbbabaacbbbbbbabbccbccaccacaabcbcccccabbcccccacbcbacbcabbccbcaabcabacbbaacbbbacacacaccbbcbaacbacaccbabbbaaacbbbaaccbaacabcbbaaabbcbaacbbacbcbacabcaaaaabbbabbccaaaccaabbbaccabcccaacacaabccaabbcaababbaabacbbccabcccaacabccbbbbbbbcccc"];
        let target = "cbcabbaabacacca".to_owned();
        let ans = 8;
        assert_eq!(
            Solution::min_valid_strings(words.into_iter().map(|s| s.to_owned()).collect(), target),
            ans
        );
    }
}

use crate::*;
struct Solution {}

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let mut arr = vec![];

        for word in words {
            let mut hash_val = StrHashVal::default();
            let mut sz = 0;
            for b in word.as_bytes() {
                hash_val.push(*b);
                sz += 1;
                if arr.len() <= sz {
                    arr.resize(sz + 1, HashSet::<StrHashVal>::default());
                }
                arr[sz].insert(hash_val.clone());
            }
        }
        let target = target.as_bytes();
        let mut hash_vals = vec![StrHashVal::default()];
        let mut hash_val = StrHashVal::default();
        for &b in target.iter() {
            hash_val.push(b);
            hash_vals.push(hash_val.clone());
        }

        let mut max_match = vec![0; target.len()];

        let mut pows = vec![];
        let mut hash_val = StrHashVal::from_str("a");
        pows.push(hash_val.clone());
        for j in 0..target.len() {
            hash_val = hash_val * StrHashVal::ALPHABET;
            pows.push(hash_val.clone());
        }

        for i in 0..target.len() {
            let mut l = 0;
            let mut r = target.len() - i + 1;
            while l + 1 < r {
                let m = (l + r) / 2;
                let v = hash_vals[i + m].clone() - hash_vals[i].clone() * pows[m].clone();
                if m < arr.len() && arr[m].contains(&v) {
                    l = m;
                } else {
                    r = m
                }
            }
            max_match[i] = l;
        }

        let mut dp = vec![i32::MAX; target.len() + 1];
        dp[0] = 0;
        let mut stk: Vec<(usize, i32)> = vec![];
        let mut ptr = 0;
        for i in 0..target.len() {
            while ptr != stk.len() && stk[ptr].0 < i {
                ptr += 1;
            }
            if ptr == stk.len() {
                if dp[i] != i32::MAX && max_match[i] != 0 {
                    dp[i + 1] = dp[i] + 1;
                    stk.push((i + max_match[i] - 1, dp[i + 1]));
                } else {
                    break;
                }
            } else {
                dp[i + 1] = stk[ptr].1;
                if max_match[i] != 0 {
                    let nxt = i + max_match[i] - 1;
                    let (j, s) = *stk.last().unwrap();
                    if s == dp[i] + 1 {
                        let v = j.max(nxt);
                        stk.last_mut().unwrap().0 = v;
                    } else if s < dp[i] + 1 && j < nxt {
                        stk.push((nxt, dp[i] + 1));
		    }
                }
            }
        }
        if dp[target.len()] == i32::MAX {
            -1
        } else {
            dp[target.len()]
        }
    }
}

use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul, Sub};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct StrHashVal {
    vals: Vec<i64>,
}

impl StrHashVal {
    const ALPHABET: i64 = 27;
    const MODULES: [i64; 1] = [1000000007];

    fn from_str<S: AsRef<str>>(s: S) -> Self {
        let mut vals = vec![];
        for module in Self::MODULES {
            let mut v = 0;
            for &c in s.as_ref().as_bytes() {
                v = (v * Self::ALPHABET) % module + (c - b'a' + 1) as i64;
            }
            vals.push(v);
        }
        Self { vals }
    }

    fn push(&mut self, byte: u8) {
        let x: i64 = (byte - b'a' + 1) as i64;
        for (v, &module) in self.vals.iter_mut().zip(Self::MODULES.iter()) {
            *v = ((*v * Self::ALPHABET) + x) % module;
        }
    }
}

impl<T> Add<T> for StrHashVal
where
    T: Into<i64>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: i64 = rhs.into();
        Self {
            vals: self
                .vals
                .into_iter()
                .zip(Self::MODULES.iter())
                .map(|(v, &m)| (v + rhs) % m)
                .collect(),
        }
    }
}

impl<T> Mul<T> for StrHashVal
where
    T: Into<i64>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: i64 = rhs.into();
        Self {
            vals: self
                .vals
                .into_iter()
                .zip(Self::MODULES.iter())
                .map(|(v, &m)| (v * rhs) % m)
                .collect(),
        }
    }
}

impl Mul for StrHashVal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            vals: self
                .vals
                .into_iter()
                .zip(rhs.vals)
                .zip(Self::MODULES.iter())
                .map(|((a, b), &m)| ((a * b) % m + m) % m)
                .collect(),
        }
    }
}

impl Sub for StrHashVal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            vals: self
                .vals
                .into_iter()
                .zip(rhs.vals)
                .zip(Self::MODULES.iter())
                .map(|((a, b), &m)| ((a - b) % m + m) % m)
                .collect(),
        }
    }
}

impl Default for StrHashVal {
    fn default() -> Self {
        Self {
            vals: vec![0; Self::MODULES.len()],
        }
    }
}
