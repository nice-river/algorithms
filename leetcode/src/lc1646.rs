struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn get_maximum_generated(n: i32) -> i32 {
		if n == 0 {
			return 0;
		}
		let n = n as usize;
		let mut dp = vec![0; n + 1];
		dp[1] = 1;
		for i in 2..=n {
			if i % 2 == 0 {
				dp[i] = dp[i / 2];
			} else {
				dp[i] = dp[i / 2] + dp[i / 2 + 1];
			}
		}
		dp.into_iter().max().unwrap()
	}
}