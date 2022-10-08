struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![1, 2, 2, 3, 1];
		assert_eq!(Solution::find_shortest_sub_array(nums), 2);
	}
}

use std::collections::HashMap;

impl Solution {
	pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
		let mut cnt = HashMap::new();
		for &num in &nums {
			*cnt.entry(num).or_insert(0) += 1;
		}
		let mut largest = HashMap::new();
		let mut maxi = 0;
		for (&k, &v) in cnt.iter() {
			if v < maxi {
				continue;
			}
			if v > maxi {
				largest.clear();
				maxi = v;
			}
			largest.insert(k, 0);
		}

		let mut start = HashMap::new();

		for (i, &num) in nums.iter().enumerate() {
			if largest.contains_key(&num) {
				let s = *start.entry(num).or_insert(i);
				largest.insert(num, i - s + 1);
			}
		}

		largest.into_iter().map(|(_, v)| v).min().unwrap() as i32
	}
}