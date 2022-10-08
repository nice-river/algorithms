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
	pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
		let mut ss = HashSet::new();
		j.chars().map(|ch| ss.insert(ch)).count();
		s.chars().filter(|ch| ss.contains(ch)).count() as i32
	}
}