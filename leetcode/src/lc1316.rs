#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let text = "abcabcabc".to_owned();
        let ans = 3;
        assert_eq!(Solution::distinct_echo_substrings(text), ans);
    }

    #[test]
    fn test1() {
        let text = "aaaaaaaaaa".to_owned();
        let ans = 5;
        assert_eq!(Solution::distinct_echo_substrings(text), ans);
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

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let text = text.as_bytes();
        let n = text.len();
        let mut set = HashSet::new();
        for i in 0..n {
            let mut kmp = vec![0];
            for j in i + 1..n {
                let mut k = j - i;
                while k > 0 && text[i + kmp[k - 1]] != text[j] {
                    k = kmp[k - 1];
                }
                if k == 0 {
                    kmp.push(if text[i] == text[j] { 1 } else { 0 });
                } else {
                    kmp.push(
                        kmp[k - 1]
                            + if text[i + kmp[k - 1]] == text[j] {
                                1
                            } else {
                                0
                            },
                    );
                }
                let a = kmp.len();
                let b = *kmp.last().unwrap();
                let c = a - b;
                if b % c == 0 && (b / c + 1) % 2 == 0 {
                    set.insert(&text[i..=j]);
                }
            }
        }
        set.len() as i32
    }
}
