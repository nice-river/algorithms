struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = 2;
		let ans = 8;
		assert_eq!(Solution::check_record(n), ans);
	}

	#[test]
	fn test1() {
		let n = 1;
		let ans = 3;
		assert_eq!(Solution::check_record(n), ans);
	}

	#[test]
	fn test2() {
		let n = 10101;
		let ans = 183236316;
		assert_eq!(Solution::check_record(n), ans);
	}

	#[test]
	fn test3() {
		let n = 4;
		let ans = 43;
		assert_eq!(Solution::check_record(n), ans);
	}
}


impl Solution {
	pub fn check_record(n: i32) -> i32 {
		if n == 1 {
			return 3;
		}
		let n = n as usize;
		let module = 10i64.pow(9) + 7;
		let mut ans = 0;
		// ALP
		let mut dp = vec![vec![vec![0; 3]; 2]; n + 1];
		dp[1][1][0] = 1;
		dp[1][0][1] = 1;
		dp[1][0][2] = 1;
		dp[2][0][1] = 2;
		dp[2][0][2] = 2;
		dp[2][1][0] = 2;
		dp[2][1][1] = 1;
		dp[2][1][2] = 1;
		for i in 3..=n {
			dp[i][0][1] = (dp[i-1][0][0] + dp[i-1][0][2] + dp[i-2][0][0] + dp[i-2][0][2]) % module;
			dp[i][0][2] = (dp[i-1][0][1] + dp[i-1][0][2]) % module;

			dp[i][1][0] = dp[i-1][0].iter().sum::<i64>() % module;
			dp[i][1][1] = (dp[i-1][1][0] + dp[i-1][1][2] + dp[i-2][1][0] + dp[i-2][1][2]) % module;
			dp[i][1][2] = dp[i-1][1].iter().sum::<i64>() % module;
		}
		(dp[n].iter().map(|arr| arr.iter().sum::<i64>() % module).collect::<Vec<i64>>().iter().sum::<i64>() % module) as i32
	}
}