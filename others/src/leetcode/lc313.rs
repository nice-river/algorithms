struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = 12;
		let primes = vec![2,7,13,19];
		let ans = 32;
		assert_eq!(Solution::nth_super_ugly_number(n, primes), ans);
	}
}


impl Solution {
	pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
		let n = n as usize;
		let mut dp = vec![1; n + 1];
		let mut pointers = vec![1; primes.len()];
		for i in 2..=n {
			let mut tmp = vec![0; primes.len()];
			let mut mini = i32::MAX;
			for j in 0..pointers.len() {
				tmp[j] = dp[pointers[j]] * primes[j];
				mini = mini.min(tmp[j]);
			}
			dp[i] = mini;
			for j in 0..tmp.len() {
				if mini == tmp[j] {
					pointers[j] += 1;
				}
			}
		}
		dp[n]
	}
}