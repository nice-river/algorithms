struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sol = Solution {};
	}
}

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
	pub fn contains_duplicate(nums: Vec<i32>) -> bool {
		let n = nums.len();
		let s: HashSet<i32> = HashSet::from_iter(nums.into_iter());
		s.len() < n
	}
}
