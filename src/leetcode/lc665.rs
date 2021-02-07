struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = [2, 4, 2, 3];
		assert!(Solution::check_possibility(nums.to_vec()));
	}
}

impl Solution {
	pub fn check_possibility(mut nums: Vec<i32>) -> bool {
		let mut k = 0;
		for i in 0..nums.len() - 1 {
			if nums[i] > nums[i + 1] {
				if k != 0 {
					return false;
				}
				if i != 0 && nums[i - 1] > nums[i + 1] {
					nums[i + 1] = nums[i];
				}
				k += 1;
			}
		}
		true
	}
}