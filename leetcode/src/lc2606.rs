#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        let mut v = vec![i32::MAX; 30];
        for (i, &c) in chars.as_bytes().iter().enumerate() {
            v[(c - b'a') as usize] = vals[i];
        }
        for c in b'a'..=b'z' {
            let k = (c - b'a') as usize;
            if v[k] == i32::MAX {
                v[k] = k as i32 + 1;
            }
        }
        let s = s
            .into_bytes()
            .into_iter()
            .map(|c| v[(c - b'a') as usize])
            .collect::<Vec<_>>();
        let mut dp = vec![0; s.len() + 1];
        let mut ans = 0;
        for i in 1..=s.len() {
            dp[i] = (dp[i - 1] + s[i - 1]).max(0);
            ans = ans.max(dp[i]);
        }
        ans
    }
}
