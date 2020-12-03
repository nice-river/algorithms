use std::collections::HashMap;
use std::cmp::{min, max};

struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![1,10000000];
		assert_eq!(Solution::maximum_gap(nums), 10000000 - 1);
	}
}

impl Solution {
	pub fn maximum_gap(nums: Vec<i32>) -> i32 {
		if nums.len() <= 1 {
			return 0;
		}
		let &mini = nums.iter().min().unwrap();
		let &maxi = nums.iter().max().unwrap();
		let mut gap = (maxi - mini) / (nums.len() - 1) as i32;
		if gap * ((nums.len() - 1) as i32) < maxi - mini {
			gap += 1;
		}
		let mut brackets = vec![(maxi + 1, mini - 1); nums.len() - 1];
		for num in nums {
			let i = ((num - mini) / (gap + 1)) as usize;
			brackets[i].0 = std::cmp::min(brackets[i].0, num);
			brackets[i].1 = std::cmp::max(brackets[i].1, num);
		}
		let mut prev = 0;
		let mut ans = brackets[0].1 - brackets[0].0;
		for i in 1..brackets.len() {
			if brackets[i].0 != maxi + 1 {
				ans = std::cmp::max(ans, brackets[i].0 - brackets[prev].1);
				prev = i;
			}
		}
		ans
	}
}