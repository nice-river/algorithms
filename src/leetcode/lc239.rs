#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![1, -1];
		let k = 1;
		assert_eq!(Solution::max_sliding_window(nums, k), vec![1, -1]);
	}
}

struct Solution {}

use std::collections::VecDeque;

impl Solution {
	pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
		let k  = k as usize;
		let mut deque = VecDeque::with_capacity(nums.len());
		let mut ans = Vec::with_capacity(nums.len());
		for i in 0..nums.len() {
			while let Some(&e) = deque.front() {
				if i - e >= k {
					deque.pop_front();
				} else {
					break;
				}
			}
			while !deque.is_empty() && nums[*deque.back().unwrap()] < nums[i] {
				deque.pop_back();
			}
			deque.push_back(i as usize);
			if i + 1 >= k {
				ans.push(nums[*deque.front().unwrap()]);
			}
		}
		ans
	}
}