struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![1, 1,];
		let k = 1;
		assert!(Solution::check_subarray_sum(nums, k));
	}
}

use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
		let mut su = 0;
		map.insert(0, -1);
		for (i, num) in nums.into_iter().enumerate(){
			let i = i as i32;
            su = (su + num) % k;
            if let Some(&p) = map.get(&su) {
                if i - p >= 2 {
					return true;
				}
			} else {
				map.insert(su, i);
			}
		}
		false
    }
}