struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn move_zeroes(nums: &mut Vec<i32>) {
		let mut j = 0;
		for i in 0..nums.len() {
			while j < nums.len() && nums[j] != 0 {
				j += 1;
			}
			if nums[i] != 0 && j < i{
				nums.swap(i, j);
			}
		}
	}
}