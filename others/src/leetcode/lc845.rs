#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
	pub fn longest_mountain(a: Vec<i32>) -> i32 {
		let n = a.len();
		if n < 3 {
			return 0;
		}
		let mut inc = vec![1; n];
		let mut dec = vec![1; n];
		for i in 1..n {
			if a[i] > a[i - 1] {
				inc[i] = inc[i - 1] + 1;
			}
		}
		for i in (0..n-1).rev() {
			if a[i] > a[i + 1] {
				dec[i] = dec[i + 1] + 1;
			}
		}
		let mut ans = 0;
		for i in 0..n {
			if inc[i] > 1 && dec[i] > 1 {
				ans = std::cmp::max(ans, inc[i] + dec[i] - 1);
			}
		}
		ans
	}
}