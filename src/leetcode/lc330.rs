struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![1, 2, 31, 33];
		let n = 2147483647;
		assert_eq!(Solution::min_patches(nums, n), 28);
	}
}

impl Solution {
	pub fn min_patches(mut nums: Vec<i32>, n: i32) -> i32 {
		let n = n as i64;
		let mut cover = 0;
		let mut ans = 0;
		let mut i = 0;
		while cover < n {
			if i < nums.len() && cover >= nums[i] as i64 - 1 {
				cover += nums[i] as i64;
				i += 1;
			} else {
				ans += 1;
				cover += cover + 1;
			}
		}
		ans
	}
}