struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::HashMap;

impl Solution {
	pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
		let mut mp = HashMap::new();
		for (i, num) in nums.iter().enumerate() {
			let r = target - num;
			if mp.contains_key(&r) {
				return vec![*mp.get(&r).unwrap() as i32, i as i32];
			}
			mp.insert(*num, i);
		}
		vec![]
	}
}