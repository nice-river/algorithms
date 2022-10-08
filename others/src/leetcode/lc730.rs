#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba".to_string();
        let ans = 104860361;
        assert_eq!(Solution::count_palindromic_subsequences(s), ans);
    }

    #[test]
    fn test1() {
        let s = "abba".to_string();
        let ans = 6;
        assert_eq!(Solution::count_palindromic_subsequences(s), ans);
    }

    #[test]
    fn test2() {
        let s = "bcbbccb".to_string();
        let ans = 14;
        assert_eq!(Solution::count_palindromic_subsequences(s), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0i64; n]; n];
        dp[0][0] = 1;
        let mut c = vec![0; 30];
        let mut p = vec![0; 30];
        let mut a = 0;
        for j in 1..n {
            dp[j][j] = 1;
            c.fill(0);
            p.fill(0);
            a = 0;
            for i in (0..j).rev() {
                if s[i] != s[j] {
                    dp[i][j] = dp[i + 1][j];
                } else {
                    if i + 1 == j {
                        dp[i][j] = 2;
                    } else {
                        dp[i][j] = (a + 2) % MOD;
                    }
                }
                let k = (s[i] - b'a') as usize;
                if p[k] == 0 {
                    p[k] = i;
                    c[k] = 1;
                    a += 1;
                } else {
                    a -= c[k];
                    c[k] = dp[i][p[k]];
                    a += c[k];
                }
            }
        }
        let mut ans = 0i64;
        p.fill(0);

        // dbg!(&dp);

        for i in (0..n).rev() {
            let k = (s[i] - b'a') as usize;
            if p[k] == 0 {
                ans = (ans + dp[0][i]) % MOD;
                p[k] = 1;
            }
        }

        ans as i32
    }
}
