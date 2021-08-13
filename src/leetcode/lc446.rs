struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![2,4,6,8,10];
		let ans = 7;
		assert_eq!(Solution::number_of_arithmetic_slices(nums), ans);
	}

	#[test]
	fn test1() {
		let nums = vec![7, 7, 7, 7];
		let ans = 5;
		assert_eq!(Solution::number_of_arithmetic_slices(nums), ans);
	}
}

use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
			let mut maps = vec![HashMap::new(); nums.len()];
			let mut ans = 0;
			for i in 1..nums.len() {
				for j in 0..i {
					let d = nums[i] as i64 - nums[j] as i64;
					let k = *maps[j].get(&d).unwrap_or(&0);
					ans += k;
					let k = k + 1 + *maps[i].get(&d).unwrap_or(&0);
					maps[i].insert(d, k);
				}
			}
			ans
    }
}