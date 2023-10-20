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
        let s = "abbb".to_owned();
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
        const ALPHA: usize = 26;
        const INVALID: i32 = i32::MAX / 2;
        let s = s.into_bytes();
        let s = s
            .into_iter()
            .map(|c| (c - b'a') as usize)
            .collect::<Vec<_>>();
        let n = s.len();
        let k = k as usize;
        if k == 0 {
            return Self::sz(&s);
        }
        if n == k {
            return 0;
        }
        let mut dp = vec![vec![vec![vec![INVALID; 11]; ALPHA]; k + 1]; n];
        dp[0][0][s[0]][1] = 1;
        for i in 1..n {
            if s[i] == s[i - 1] {
                for j in 2..=8 {
                    dp[i][0][s[i]][j + 1] = dp[i - 1][0][s[i]][j];
                }
                dp[i][0][s[i]][2] = dp[i][0][s[i]][2].min(dp[i - 1][0][s[i]][1] + 1);
                dp[i][0][s[i]][10] = dp[i - 1][0][s[i]][10].min(dp[i - 1][0][s[i]][9] + 1);
                dp[i][0][s[i]][10] = dp[i - 1][0][s[i]][10].min(dp[i - 1][0][s[i]][10]);
            } else {
                for c in 0..ALPHA {
                    for p in 0..=10 {
                        dp[i][0][s[i]][1] = dp[i][0][s[i]][1].min(dp[i - 1][0][c][p] + 1);
                    }
                }
            }
        }
        for i in 0..n {
            if k >= i + 1 {
                for j in 0..ALPHA {
                    dp[i][i + 1][j][0] = 0;
                }
            }
        }
        for i in 1..n {
            for j in 1..=i.min(k) {
                for k in 0..ALPHA {
                    let c = s[i];
                    for p in 0..=10 {
                        dp[i][j][k][p] = dp[i][j][k][p].min(dp[i - 1][j - 1][k][p]);
                    }
                    if c == k {
                        for p in 3..=9 {
                            dp[i][j][c][p] = dp[i][j][c][p].min(dp[i - 1][j][c][p - 1]);
                        }
                        dp[i][j][c][1] = dp[i][j][c][1].min(dp[i - 1][j][c][0] + 1);
                        dp[i][j][c][2] = dp[i][j][c][2].min(dp[i - 1][j][c][1] + 1);
                        dp[i][j][c][10] = dp[i][j][c][10].min(dp[i - 1][j][c][9] + 1);
                        dp[i][j][c][10] = dp[i][j][c][10].min(dp[i - 1][j][c][10]);
                    } else {
                        dp[i][j][c][1] = dp[i][j][c][1].min(INVALID);
                        for p in 0..=10 {
                            dp[i][j][c][1] = dp[i][j][c][1].min(dp[i - 1][j][k][p] + 1);
                        }
                    }
                }
            }
        }
        // for i in 9..n {
        //     for j in 0..=k {
        //         dbg!((i, j, &dp[i][j]));
        //     }
        // }
        let mut ans = INVALID;
        for i in 0..ALPHA {
            for j in 0..=10 {
                ans = ans.min(dp[n - 1][k][i][j]);
            }
        }
        ans
    }

    fn sz(arr: &[usize]) -> i32 {
        if arr.len() == 0 {
            return 0;
        }
        let mut x = 1;
        let mut ret = 1;
        for i in 1..arr.len() {
            if arr[i] == arr[i - 1] {
                if x == 1 || x == 9 || x == 99 {
                    ret += 1;
                }
                x += 1;
            } else {
                ret += 1;
                x = 1;
            }
        }
        ret
    }
}
