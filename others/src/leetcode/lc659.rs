struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![1, 2, 3, 3, 4, 5];
		assert!(Solution::is_possible(nums));
	}
}

use std::collections::HashMap;

impl Solution {
	pub fn is_possible(nums: Vec<i32>) -> bool {
		let mut cnts = HashMap::new();
		for &num in &nums {
			*cnts.entry(num).or_insert(0) += 1;
		}
		let mut cnts: Vec<(i32, i32)> = cnts.into_iter().collect();
		cnts.sort();

		let mut p = 0;
		while p < cnts.len() {
			if cnts[p].1 == 0 {
				p += 1;
				continue;
			}
			let mut j = p + 1;
			let mut t = 1;
			cnts[p].1 -= 1;
			while j < cnts.len() {
				if cnts[j].0 != cnts[j - 1].0 + 1 || cnts[j].1 < cnts[j - 1].1 + 1 {
					break;
				}
				cnts[j].1 -= 1;
				t += 1;
				j += 1;
			}
			if t < 3 {
				return false;
			}
		}
		true
	}
}