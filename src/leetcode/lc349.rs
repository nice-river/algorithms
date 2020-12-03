struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::HashSet;

impl Solution {
	pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
		let a = nums1.into_iter().collect::<HashSet<_>>();
		let b = nums2.into_iter().collect::<HashSet<_>>();
		a.intersection(&b).into_iter().map(|e| *e).collect::<Vec<i32>>()
	}
}