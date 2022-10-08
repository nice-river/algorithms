#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "5F3Z-2e-9-w".to_string();
        let k = 4;
        let ans = "5F3Z-2E9W".to_string();
        assert_eq!(Solution::license_key_formatting(s, k), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let s = s.as_bytes();
        let mut cnt = 0;
        for b in s {
            if *b != b'-' {
                cnt += 1;
            }
        }
        let p = cnt / k;
        let q = cnt % k;
        let mut i = 0;
        let mut ans = String::new();
        for _ in 0..q {
            while i < s.len() && s[i] == b'-' {
                i += 1;
            }
            ans.push((s[i] as char).to_ascii_uppercase());
            i += 1;
        }
        if q != 0 {
            ans.push('-');
        }
        for _ in 0..p {
            for _ in 0..k {
                while i < s.len() && s[i] == b'-' {
                    i += 1;
                }
                ans.push((s[i] as char).to_ascii_uppercase());
                i += 1;
            }
            ans.push('-');
        }
        ans.pop();
        ans
    }
}
