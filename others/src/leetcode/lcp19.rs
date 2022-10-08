struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let leaves = "rrryyyrryyyrr".to_string();
		assert_eq!(Solution::minimum_operations(leaves), 2);
	}
}

use std::cmp::min;

impl Solution {
	pub fn minimum_operations(leaves: String) -> i32 {
		let MAX = 1 << 30;
		let leaves = leaves.chars().collect::<Vec<char>>();
		let n = leaves.len();
		let mut mem = vec![vec![MAX; n], vec![MAX; n]];

		match leaves[n-1] {
			'r' => mem[0][n-1] = 0,
			'y' => mem[0][n-1] = 1,
			_ => unreachable!()
		}
		for i in (0..n - 1).rev() {
			match leaves[i] {
				'r' => {
					mem[0][i] = mem[0][i+1];
					mem[1][i] = min(mem[0][i+1], mem[1][i+1]) + 1;
				}
				'y' => {
					mem[0][i] = mem[0][i+1] + 1;
					mem[1][i] = min(mem[0][i+1], mem[1][i+1]);
				}
				_ => unreachable!()
			}
		}
		let mut ans = 1 << 30;
		let mut dp = vec![vec![MAX; n], vec![MAX; n]];
		match leaves[0] {
			'r' => dp[0][0] = 0,
			'y' => dp[0][0] = 1,
			_ => unreachable!()
		}
		for i in 1..n {
			ans = min(ans, min(dp[0][i-1], dp[1][i-1]) + mem[1][i]);
			ans = min(ans, dp[1][i-1] + min(mem[0][i], mem[1][i]));
			match leaves[i] {
				'r' => {
					dp[0][i] = dp[0][i-1];
					dp[1][i] = min(dp[0][i-1], dp[1][i-1]) + 1;
				}
				'y' => {
					dp[0][i] = dp[0][i-1] + 1;
					dp[1][i] = min(dp[0][i-1], dp[1][i-1]);
				}
				_ => unreachable!()
			}
		}
		ans
	}
}