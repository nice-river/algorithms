struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test0() {
		let nums = vec![1, 2, 3, 4];
		let ans = 2;
		assert_eq!(Solution::find_lhs(nums), ans);
	}


	#[test]
	fn test1() {
		let nums = vec![2,1,2,3,1,3,3];
		let ans = 5;
		assert_eq!(Solution::find_lhs(nums), ans);
	}
}

use std::collections::HashMap;

impl Solution {
	pub fn find_lhs(nums: Vec<i32>) -> i32 {
		let mut map = HashMap::new();
		for num in nums {
			*map.entry(num).or_insert(0) += 1;
		}
		let mut ans = 0;
		for (&num, &x) in map.iter() {
			if let Some(&y) = map.get(&(num + 1)) {
				ans = ans.max(x + y);
			}
		}
		ans
	}
}
