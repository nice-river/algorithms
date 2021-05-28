struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "dabcacb";
		assert_eq!(Solution::strange_printer(s.to_string()), 5);
	}

	#[test]
	fn test2() {
		let s = "a";
		assert_eq!(Solution::strange_printer(s.to_string()), 1);
	}
}

impl Solution {
	pub fn strange_printer(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![vec![i32::MAX; s.len()]; s.len()];
		for i in (0..s.len()).rev() {
            dp[i][i] = 1;
			for j in i+1..s.len() {
				if s[i] == s[j] {
					dp[i][j] = dp[i][j-1];
				} else {
					for k in i..j {
						dp[i][j] = dp[i][j].min(dp[i][k] + dp[k+1][j]);
					}
				}
			}
		}
		dp[0][s.len() - 1]
	}
}