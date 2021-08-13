struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "abc".to_string();
		let ans = 1;
		assert_eq!(Solution::longest_palindrome_subseq(s), ans);
	}
}

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
			let s = s.as_bytes();
			let n = s.len();
			let mut dp = vec![vec![0; n + 1]; 2];
			let mut cur = 0;

			for i in 0..n {
				let nxt = cur ^ 1;
				for j in 0..n {
					dp[nxt][j+1] = dp[cur][j+1].max(dp[nxt][j]);
					if s[i] == s[n-1-j] {
						dp[nxt][j+1] = dp[nxt][j+1].max(dp[cur][j] + 1);
					}
				}
				cur ^= 1;
			}

			dp[cur][n]
    }
}