struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
	pub fn unique_occurrences(arr: Vec<i32>) -> bool {
		let mut cnts = HashMap::new();
		for num in arr {
			*cnts.entry(num).or_insert(0) += 1;
		}
		let n = cnts.len();
		let mut s = HashSet::new();
		for (k, v) in cnts.into_iter() {
			s.insert(v);
		}
		n == s.len()
	}
}