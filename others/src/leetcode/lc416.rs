struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::can_partition(vec![1, 2, 5]), false);
	}
}

impl Solution {
	pub fn can_partition(nums: Vec<i32>) -> bool {
		let s: i32 = nums.iter().sum::<i32>();
		if s % 2 != 0 {
			return false;
		}
		let s = s / 2;
		let mut dp = vec![false; s as usize + 1];
		dp[0] = true;
		for i in 0..nums.len() {
			for j in (0..dp.len()).rev() {
				if dp[j] && j + nums[i] as usize <= s as usize {
					dp[j + nums[i] as usize] = true;
				}
			}
		}
		*dp.last().unwrap()
	}
}
