#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "1*".into();
        let ans = 18;
        assert_eq!(Solution::num_decodings(s), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.into_bytes();
        let n = s.len();
        if n == 1 {
            return Solution::calc_one_char(&s, 0).max(0) as i32;
        }
        let mut ans = 0;
        let module = 10i64.pow(9) + 7;
        let mut dp = vec![Solution::calc_one_char(&s, n - 1), 1];

        for i in (0..n - 1).rev() {
            let a = Solution::calc_one_char(&s, i) * dp[0] % module;
            let b = Solution::calc_two_char(&s, i) * dp[1] % module;
            dp[1] = dp[0];
            dp[0] = (a + b) % module;
        }
        dp[0] as i32
    }

    fn calc_one_char(s: &Vec<u8>, idx: usize) -> i64 {
        if s[idx] == b'0' {
            return 0;
        }
        if s[idx] == b'*' {
            9
        } else {
            1
        }
    }

    fn calc_two_char(s: &Vec<u8>, idx: usize) -> i64 {
        if s[idx] == b'0' {
            return 0;
        }
        match (s[idx], s[idx + 1]) {
            (b'*', b'*') => 15,
            (x, b'*') => {
                if x == b'1' {
                    9
                } else if x == b'2' {
                    6
                } else {
                    0
                }
            }
            (b'*', y) => {
                if y >= b'7' {
                    1
                } else {
                    2
                }
            }
            _ => {
                let num = std::str::from_utf8(&s[idx..idx + 2]).unwrap();
                let num = num.parse::<i64>().unwrap();
                if num > 26 {
                    0
                } else {
                    1
                }
            }
        }
    }
}
