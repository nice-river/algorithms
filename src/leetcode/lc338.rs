struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn count_bits(num: i32) -> Vec<i32> {
		let num = num as usize;
		let mut ans = vec![0; num + 1];
		for i in 1..=num {
			ans[i] = ans[i >> 1] + (i & 1) as i32;
		}
		ans
	}
}