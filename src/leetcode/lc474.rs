#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
	pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
		let m = m as usize;
		let n = n as usize;
		let mut dp = vec![vec![vec![0; n + 1]; m + 1]; 2];
		let mut cur = 0;
		for str in strs.iter() {
			let nxt = cur ^ 1;
			let mut zero = 0;
			let mut one = 0;
			for &b in str.as_bytes() {
				if b == b'0' {
					zero += 1;
				} else {
					one += 1;
				}
			}
			dp[nxt] = dp[cur].clone();
			for i in zero..=m {
				for j in one..=n {
					dp[nxt][i][j] = dp[nxt][i][j].max(dp[cur][i-zero][j-one] + 1);
				}
			}
			cur ^= 1;

		}
		dp[cur][m][n]
	}
}
