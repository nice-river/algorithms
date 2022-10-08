struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let a = vec![0, 0, 0, 1, 0, 1, 1, 0];
		let k = 3;
		assert_eq!(Solution::min_k_bit_flips(a, k), 3);

		let a = vec![0, 1, 0];
		let k = 1;
		assert_eq!(Solution::min_k_bit_flips(a, k), 2);

		let a = vec![0, 1, 0];
		let k = 2;
		assert_eq!(Solution::min_k_bit_flips(a, k), 2);

		let a = vec![0, 1, 1];
		let k = 2;
		assert_eq!(Solution::min_k_bit_flips(a, k), -1);
	}
}

impl Solution {
	pub fn min_k_bit_flips(a: Vec<i32>, k: i32) -> i32 {
		let k = k as usize;
		let mut map = vec![0; a.len()];
		let mut ans = 0;
		for i in 0..a.len() + 1 - k {
			map[i + k - 1] = ans;
			let d = ans - map[i];
			if (a[i] + d) % 2 == 0 {
				ans += 1;
			}
		}
		for i in a.len() - k + 1..a.len() {
			let d = ans - map[i];
			if (a[i] + d) % 2 == 0 {
				return -1;
			}
		}
		ans
	}
}