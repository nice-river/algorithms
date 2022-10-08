struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sol = Solution {};
	}
}

impl Solution {
	pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
		let text1 = text1.as_bytes();
		let text2 = text2.as_bytes();
		let n = text1.len();
		let m = text2.len();
		let mut dp = vec![vec![0; m + 1]; n + 1];

		for i in 0..n {
			for j in 0..m {
				dp[i+1][j+1] = dp[i+1][j].max(dp[i][j+1]);
				if text1[i] == text2[j] {
					dp[i+1][j+1] = dp[i+1][j+1].max(dp[i][j] + 1);
				}
			}
		}
		dp[n][m]
	}
}