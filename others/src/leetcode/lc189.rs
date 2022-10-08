struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn rotate(nums: &mut Vec<i32>, k: i32) {
		let k = k as usize % nums.len();
		nums.rotate_right(k);
	}
}