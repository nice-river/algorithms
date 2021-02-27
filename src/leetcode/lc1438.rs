struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![8, 2, 4, 7];
		let limit = 4;
		assert_eq!(Solution::longest_subarray(nums, limit), 2);

		let nums = vec![10, 1, 2, 4, 7, 2];
		let limit = 5;
		assert_eq!(Solution::longest_subarray(nums, limit), 4);

		let nums = vec![4, 2, 2, 2, 4, 4, 2, 2];
		let limit = 0;
		assert_eq!(Solution::longest_subarray(nums, limit), 3);

	}
}

use std::collections::VecDeque;

impl Solution {
	pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
		let mut i = 0;
		let mut j = 0;
		let mut mini_stk = VecDeque::with_capacity(nums.len());
		let mut maxi_stk = VecDeque::with_capacity(nums.len());
		while j < nums.len() {
			while !mini_stk.is_empty() && mini_stk.iter().next_back().unwrap() > &nums[j] {
				mini_stk.pop_back();
			}
			mini_stk.push_back(nums[j]);

			while !maxi_stk.is_empty() && maxi_stk.iter().next_back().unwrap() < &nums[j] {
				maxi_stk.pop_back();
			}
			maxi_stk.push_back(nums[j]);

			j += 1;
			let &a = mini_stk.iter().next().unwrap();
			let &b =  maxi_stk.iter().next().unwrap();
			if (a - b).abs() > limit {
				if a == nums[i] {
					mini_stk.pop_front();
				}
				if b == nums[i] {
					maxi_stk.pop_front();
				}
				i += 1;
			}
		}
		(j - i) as i32
	}
}