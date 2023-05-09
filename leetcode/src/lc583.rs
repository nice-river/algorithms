#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
	pub fn min_distance(word1: String, word2: String) -> i32 {
		let word1 = word1.as_bytes();
		let word2 = word2.as_bytes();
		let n = word1.len();
		let m = word2.len();
		let mut dp = vec![vec![0; m + 1]; n + 1];
		for i in 1..=n {
			for j in 1..=m {
				dp[i][j] = dp[i][j-1].max(dp[i-1][j]);
				if word1[i-1] == word2[j-1] {
					dp[i][j] = dp[i][j].max(dp[i-1][j-1] + 1);
				}
			}
		}
		(n + m - dp[n][m] * 2) as i32
	}
}