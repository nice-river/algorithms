struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let people = vec![1, 2];
		let limit = 3;
		let ans = 1;
		assert_eq!(Solution::num_rescue_boats(people, limit), ans);
	}
}

use std::collections::BinaryHeap;

impl Solution {
	pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
		people.sort_unstable();
		let mut ans = 0;
		let (mut l, mut r) = (0, people.len());
		while l < r {
			if people[l] + people[r-1] <= limit {
				l += 1;
				r -= 1;
			} else {
				r -= 1;
			}
			ans += 1;
		}
		ans
	}
}
