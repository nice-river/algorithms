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
	pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
		let mut map = HashMap::new();
		let mut ans = vec![];
		for str in &strs {
			let mut t = str.clone().into_bytes();
			t.sort();
			let t = std::str::from_utf8(&t).unwrap().to_string();
			map.entry(t).or_insert(Vec::new()).push(str.clone());
		}
		for (_, v) in map.into_iter() {
			ans.push(v);
		}
		ans
	}
}