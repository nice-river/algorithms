#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "aacc".to_owned();
        let k = 1;
        let ans = 3;
        assert_eq!(Solution::get_length_of_optimal_compression(s, k), ans);
    }

    #[test]
    fn test5() {
        let s = "aaabaaaabbbaaabbbbbbbbbaaababbbbabbbabbbbbaaaaabab".to_owned();
        let k = 8;
        let ans = 12;
        assert_eq!(Solution::get_length_of_optimal_compression(s, k), ans);
    }

    #[test]
    fn test6() {
        let s = "aabbaabba".to_owned();
        let k = 4;
        let ans = 2;
        assert_eq!(Solution::get_length_of_optimal_compression(s, k), ans);
    }

    #[test]
    fn test3() {
        let s = "abb".to_owned();
        let k = 1;
        let ans = 2;
        assert_eq!(Solution::get_length_of_optimal_compression(s, k), ans);
    }

    #[test]
    fn test4() {
        let s = "aaaabbbbb".to_owned();
        let k = 4;
        let ans = 2;
        assert_eq!(Solution::get_length_of_optimal_compression(s, k), ans);
    }

    #[test]
    fn test1() {
        let s = "aabbbbbaa".to_owned();
        let k = 4;
        let ans = 2;
        assert_eq!(Solution::get_length_of_optimal_compression(s, k), ans);
    }

    #[test]
    fn test2() {
        let s = "babbbbbbbbb".to_owned();
        let k = 1;
        let ans = 3;
        assert_eq!(Solution::get_length_of_optimal_compression(s, k), ans);
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

use crate::*;

struct Solution {}

impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let k = k as usize;
        let mut dp = vec![vec![i32::MAX / 2; k + 1]; n + 1];
        dp[0][0] = 0;
        for i in 1..=n {
            for j in 0..=i.min(k) {
                if j > 0 {
                    dp[i][j] = dp[i - 1][j - 1];
                }
                let mut same = 1;
                let mut diff = 0;
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
                for k in (1..i).rev() {
                    if s[k - 1] == s[i - 1] {
                        same += 1;
                        dp[i][j] = dp[i][j].min(dp[k - 1][j - diff] + Self::sz(same));
                    } else {
                        diff += 1;
                    }
                    if diff > j {
                        break;
                    }
                }
            }
        }
        dp[n][k]
    }

    fn sz(n: usize) -> i32 {
        match n {
            1 => 1,
            2..=9 => 2,
            10..=99 => 3,
            _ => 4,
        }
    }
}
