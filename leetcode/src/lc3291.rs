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

    #[test]
    fn test6() {
        let words = ["abaaabbabaaaaabbbabaabbaaaaaababaaabaaaabaab","baabbbbaababaaabbbbbbaabbbbbaaabbaabbabaaaaaabbbbbbbaabbabbbaabbabbaabaaaabbaaabaaaabaaaaabaaaabaaaa","abbabababaabbababbabaababbbaababbaaaaabaaabaaabaaabbaaaaaabaababbabbabbaaababaabaabaabbaaaabbbbaaa","aabbbaaabbbbbabaa","bbbbbbabbbb","aabaabbbabba","aabaaabbbababbbaabbbaabbbababaabbabbaaabaabbababbbaab","baaabaababab","abbbaaaaaabbbaabaabbbaaaaabaaabbababababbbabaabaababaaaaaabaabbababbabbbaabbbabbbaaaababaababaaababbaaaabbbaabaaaaabaabbbbaaaababbaabbbaaaabaababbaabbbbaaabbabaabbbbabaaababbababbaabbbabaabaaabaabbbaabbaaabaaaabababbabbaaabbbbabbabaabaababbbabbabbaaaabbaabbbabababbbbababbbabaaaabaabaaabbabbbbbaabaaabbbbbbbbaaababaaaabaaabababbaabaabbaaaaabbaaabababaabbbaaaabbbabbbaabaabbbbbaababbbabababbbaabbbbabaabbaabbbaabbbbaabaaaaabbabbabbbabbbbabbaabbabbbbaabaabbabbbabababbaabbbbaaaaabaaabbabbabbbbbababbbbaaabaaaaabaaaaabaaaabaaababaabaabbbbaabbaaaaabbbaabbabbbbbabababaabbbaaaabbbbabbabbababaaaabbabbbaabbbbbaabbbaabababbabbbaaabababaaababbbbbbabbabababaabbaabaaaaabaaabbaaaaabaabbbaabbbbbabbbabbababbbbaabbbbbabbbbbaaaaababbbabbbbbaaabaaabaaabbaaabbbbbbaabaabbaaabbabbabbaabbabbaabbaabbbabaababbbbaabaaaabaaaaabbabbbbbbbbbbaabbbaaaaaabbaaaababbbaabbbbbbabaaaabbaaaaababaababaaaaabababbaaaaaaaaaabbaaaaaababbbbbbabaabbabaaaaaababbbbabbbabaaabaabaaaaaababaaaaabbbaaaaababaaabaaaabbabbbaaaaabaaaabbbbbbabbbbaabbbbbbbbbaabababbabbaabbbbbbbbabbababbbabaaababbbbababbaaaabaabbaaaabaaababbbbaabbbbbbabbabbabbbaabaabbaaaababaababbbaaaabbbbaababbabbbaababababaaaaaabbababaabbbaaabbabbaaabbaabbbabbbbababbababbaabaabbbbbbbabbabaaabaaaaaabaabbaabbabbaabaaaaabaabbbabaaaabaabaaaaaabaababaabbbbababbaabbbaababbbbbbbbaaabaaabaaabbbbaaabbbabaaabaaabbbabaaabbabbabababaaabbabaabbbbabababbbaaaaabaaaaaaabaaaaabbbbaababbbabbbbbbbababbaabbbbbbbbbabbabaabaababbbabaaabaabbabbbaaabaaaaababbbaabbbaabaabbabbbaabbbbabaababaaaaababbbabaaaaaaaabbbaaaaabaababbbaabbabababaabbbbbbaabbaabbabababababbaabbaaabbaabaaaababbaaabbabbabbbaabbabababbbbabbbabbabbabababbbabbbaabbbabbbbaabbbabbbbaaaabaaaabaaaabababaabbbaaabbabbabbaaaaabbaabbbabbbabbbbbbaaaaabaaabbbaaaabaaabbbbbbbaabaaababaaaaabbbabbabaabbbbbbbbaaaaabaabaabaaaabbbbaaaabaaabababbabbbbaabbbabbbbbabbba","aabbaaabababbababaaaaabbbaba"];
        let target = "aaaabbabbabbaaaaaaabbbabaaabbbabbbbaabbbbbaabababb".to_owned();
        let ans = 13;
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

        const BASE: i64 = 27;
        const MODULES: [i64; 2] = [1000000007, 2147482937];
        type HashVals = Vector<{ MODULES.len() }>;

        for word in words {
            let mut hash_val = HashVals::default();
            let mut sz = 0;
            for &b in word.as_bytes() {
                hash_val = (hash_val * BASE + (b - b'a') as i64 + 1) % MODULES;
                sz += 1;
                if arr.len() <= sz {
                    arr.resize(sz + 1, HashSet::<HashVals>::default());
                }
                arr[sz].insert(hash_val);
            }
        }
        let target = target.as_bytes();
        let mut hash_vals = vec![HashVals::default()];
        let mut hash_val = HashVals::default();
        for &b in target.iter() {
            hash_val = (hash_val * BASE + (b - b'a') as i64 + 1) % MODULES;
            hash_vals.push(hash_val);
        }

        let mut max_match = vec![0; target.len()];

        let mut pows = vec![];
        let mut hash_val = HashVals::from_int(1);
        pows.push(hash_val);
        for j in 0..target.len() {
            hash_val = hash_val * BASE % MODULES;
            pows.push(hash_val);
        }
        for i in 0..target.len() {
            let mut l = 0;
            let mut r = target.len() - i + 1;
            while l + 1 < r {
                let m = (l + r) / 2;
                let v = ((hash_vals[i + m] - hash_vals[i] * pows[m] % MODULES) % MODULES + MODULES)
                    % MODULES;
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
use std::ops::{Add, Mul, Rem, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Vector<const N: usize> {
    vals: [i64; N],
}

impl<const N: usize> Vector<N> {
    fn new() -> Self {
        Self { vals: [0; N] }
    }

    fn from_int<T: Into<i64>>(x: T) -> Self {
        let x: i64 = x.into();
        Self { vals: [x; N] }
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! impl_add_vector {
    ($integer:ty) => {
        impl<const N: usize> Add<[$integer; N]> for Vector<N> {
            type Output = Self;

            fn add(mut self, rhs: [$integer; N]) -> Self::Output {
                self.vals
                    .iter_mut()
                    .zip(rhs)
                    .for_each(|(v, r)| *v += r as i64);
                self
            }
        }
    };
}

impl_add_vector!(u8);
impl_add_vector!(i32);
impl_add_vector!(i64);

macro_rules! impl_add_scala {
    ($integer:ty) => {
        impl<const N: usize> Add<$integer> for Vector<N> {
            type Output = Self;

            fn add(mut self, rhs: $integer) -> Self::Output {
                let rhs: i64 = rhs as i64;
                self.vals.iter_mut().for_each(|v| *v += rhs);
                self
            }
        }
    };
}

impl_add_scala!(u8);
impl_add_scala!(i32);
impl_add_scala!(i64);

impl<T, const N: usize> Mul<T> for Vector<N>
where
    T: Into<i64>,
{
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        let rhs: i64 = rhs.into();
        self.vals.iter_mut().for_each(|v| *v *= rhs);
        self
    }
}

impl<const N: usize> Mul for Vector<N> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.vals
            .iter_mut()
            .zip(rhs.vals)
            .for_each(|(v, rhs)| *v *= rhs);
        self
    }
}

impl<const N: usize> Sub for Vector<N> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.vals
            .iter_mut()
            .zip(rhs.vals)
            .for_each(|(v, rhs)| *v -= rhs);
        self
    }
}

impl<T, const N: usize> Rem<[T; N]> for Vector<N>
where
    T: Into<i64>,
{
    type Output = Self;

    fn rem(mut self, rhs: [T; N]) -> Self::Output {
        self.vals
            .iter_mut()
            .zip(rhs)
            .for_each(|(v, r)| *v %= r.into());
        self
    }
}
