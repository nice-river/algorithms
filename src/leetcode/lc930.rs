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
	pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
		let mut map = HashMap::new();
		map.insert(0, 1);
		let mut ans = 0;
		let mut s = 0;
		for num in nums {
			s += num;
			if let Some(&x) = map.get(&(s - goal)) {
				ans += x;
			}
            *map.entry(s).or_insert(0) += 1;
		}
		ans
	}
}