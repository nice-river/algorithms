#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = 2;
		let ans = 2;
		assert_eq!(Solution::count_arrangement(n), ans);
	}
}

struct Solution {}

impl Solution {
	pub fn count_arrangement(n: i32) -> i32 {
		let mut dp = vec![0; 1 << n];
		dp[0] = 1;
		for i in 0usize..(1 << n) {
			let c = i.count_ones() as i32;
			for j in 0..n {
				if i & (1 << j) != 0 && ((j + 1) % c == 0 || c % (j + 1) == 0) {
					dp[i] += dp[i ^ (1 << j)]
				}
			}
		}
		dp[(1 << n) - 1]
	}
}