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
	pub fn word_pattern(pattern: String, s: String) -> bool {
		let mut map = HashMap::new();
		let mut re_map = HashMap::new();
		let pattern = pattern.into_bytes();
		let words = s.split(" ").collect::<Vec<&str>>();
		if pattern.len() != words.len() {
			return false;
		}
		for i in 0..pattern.len() {
			if map.get(&pattern[i]).is_none() {
				if re_map.get(&words[i]).is_none() {
					map.insert(pattern[i], words[i]);
					re_map.insert(words[i], pattern[i]);
				} else {
					return false;
				}
			} else if map.get(&pattern[i]).unwrap() != &words[i] {
				return false;
			}
		}
		true
	}
}