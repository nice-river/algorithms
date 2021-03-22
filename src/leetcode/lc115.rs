struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "rabbbit";
		let t = "rabbit";
		assert_eq!(Solution::num_distinct(s.to_string(), t.to_string()), 3);
	}

	#[test]
	fn test1() {
		let s = "babgbag";
		let t = "bag";
		assert_eq!(Solution::num_distinct(s.to_string(), t.to_string()), 5);
	}

	#[test]
	fn test2() {
		let s = "aa";
		let t = "a";
		assert_eq!(Solution::num_distinct(s.to_string(), t.to_string()), 2);
	}

	#[test]
	fn test3() {
		let s = "aa";
		let t = "aa";
		assert_eq!(Solution::num_distinct(s.to_string(), t.to_string()), 1);
	}

	#[test]
	fn test4() {
		let s = "aaaa";
		let t = "aa";
		assert_eq!(Solution::num_distinct(s.to_string(), t.to_string()), 6);
	}

}

impl Solution {
	pub fn num_distinct(s: String, t: String) -> i32 {
		let s = s.as_bytes();
		let t = t.as_bytes();
		let m = s.len();
		let n = t.len();
		if m < n {
			return 0;
		}
		let mut dp = vec![vec![0; m + 1]; n + 1];
		for j in 0..=m {
			dp[0][j] = 1;
		}
		for i in 1..=n {
			for j in 1..=m {
				dp[i][j] = dp[i][j-1];
				if t[i-1] == s[j-1] {
					dp[i][j] += dp[i - 1][j - 1];
				}
			}
		}
		dp[n][m]
	}
}