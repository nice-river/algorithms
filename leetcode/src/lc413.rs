struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![1, 2, 3, 4];
		let ans = 3;
		assert_eq!(Solution::number_of_arithmetic_slices(nums), ans);
	}
}

impl Solution {
	pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
		if nums.len() < 3 {
			return 0;
		}
		let mut ans = 0;
		let mut i = 0;
		while i < nums.len() - 2 {
			let mut j = i + 2;
			while j < nums.len() && nums[j] - nums[j-1] == nums[j-1] - nums[j-2] {
				j += 1;
			}
			if j - i >= 3 {
				let k = (j - i - 2) as i32;
				ans += (k + 1) * k / 2;
			}
			i = j - 1;
		}
		ans
	}
}