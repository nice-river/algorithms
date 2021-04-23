struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::HashSet;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![-1; s.len()];

        let mut set = HashSet::new();

        for ch in 1..=26 {
            set.insert(ch.to_string());
        }

        Solution::dfs(0, s, &mut dp, &set)
    }

    fn dfs(idx: usize, s: &[u8], dp: &mut Vec<i32>, set: &HashSet<String>) -> i32 {
        if idx == s.len() {
            return 1;
        }
        if dp[idx] != -1 {
            return dp[idx];
        }
        let mut ret = 0;

        for next_idx in idx+1..=s.len().min(idx+2) {
            let k = std::str::from_utf8(&s[idx..next_idx]).unwrap();
            if set.contains(k) {
                ret += Solution::dfs(next_idx, s, dp, set);
            }
        }

        dp[idx] = ret;
        ret
    }
}