#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "011001111111101001010000001010011";
        let ans = 25;
        assert_eq!(Solution::minimum_time(s.to_string()), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn minimum_time(s: String) -> i32 {
        let mut s = s.into_bytes();
        let mut dp0 = vec![0; s.len()];
        let mut dp1 = vec![0; s.len()];
        let mut ones = 0;
        for i in 0..s.len() {
            if s[i] == b'1' {
                ones += 1;
                dp0[i] = (i as i32 + 1).min(*dp0.get(i.wrapping_sub(1)).unwrap_or(&0) + 2);
            } else if i > 0 {
                dp0[i] = dp0[i - 1];
            }
        }
        ones = 0;
        for i in (0..s.len()).rev() {
            if s[i] == b'1' {
                ones += 1;
                dp1[i] = ((s.len() - i) as i32).min(*dp1.get(i + 1).unwrap_or(&0) + 2);
            } else if i < s.len() - 1 {
                dp1[i] = dp1[i + 1];
            }
        }
        let mut ans = dp1[0].min(dp0[s.len() - 1]);
        for i in 1..s.len() {
            ans = ans.min(dp0[i - 1] + dp1[i]);
        }
        ans
    }
}
