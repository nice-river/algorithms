#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let word1 = "bcca";
        let word2 = "abc";
        let ans = 1;
        assert_eq!(
            Solution::valid_substring_count(word1.to_owned(), word2.to_owned()),
            ans
        );
    }

    #[test]
    fn test1() {
        let word1 = "abcabc";
        let word2 = "abc";
        let ans = 10;
        assert_eq!(
            Solution::valid_substring_count(word1.to_owned(), word2.to_owned()),
            ans
        );
    }

    #[test]
    fn test2() {
        let word1 = "aaaaaaa";
        let word2 = "a";
        let ans = 21;
        assert_eq!(
            Solution::valid_substring_count(word1.to_owned(), word2.to_owned()),
            ans
        );
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut base = [0; 26];
        for &c in word2.as_bytes() {
            base[(c - b'a') as usize] += 1;
        }
        let mut cnt = [0; 26];
        let mut left = 0;
        let mut right = 0;
        let word1 = word1.as_bytes();
        let mut ans = 0;
        while right < word1.len() {
            if Self::cover(&cnt, &base) {
                ans += word1.len() - right + 1;
                cnt[(word1[left] - b'a') as usize] -= 1;
                left += 1;
            } else {
                cnt[(word1[right] - b'a') as usize] += 1;
                right += 1;
            }
        }
        while Self::cover(&cnt, &base) {
            ans += 1;
            cnt[(word1[left] - b'a') as usize] -= 1;
            left += 1;
        }
        ans as i64
    }

    fn cover(a: &[i32], b: &[i32]) -> bool {
        a.iter().zip(b.iter()).all(|(x, y)| x >= y)
    }
}
