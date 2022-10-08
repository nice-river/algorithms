struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn pivot_index(nums: Vec<i32>) -> i32 {
		let s = nums.iter().fold(0, |a, &b| a + b);
		let mut h = 0;
		for (i, num) in nums.into_iter().enumerate() {
			if h == s - h - num {
				return i as i32;
			}
			h += num;
		}
		-1
	}
}