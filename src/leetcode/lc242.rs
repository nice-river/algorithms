struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sol = Solution {};
	}
}

use std::collections::BTreeMap;

impl Solution {
	pub fn is_anagram(s: String, t: String) -> bool {
		let mut sm = BTreeMap::new();
		let mut tm = BTreeMap::new();
		for c in s.chars() {
			*sm.entry(c).or_insert(0) += 1;
		}
		for c in t.chars() {
			*tm.entry(c).or_insert(0) += 1;
		}
		for ((&sk, &sv), (&tk, &tv)) in sm.iter().zip(tm.iter()) {
			if sk != tk || sv != tv {
				return false;
			}
		}
		sm.len() == tm.len()
	}
}