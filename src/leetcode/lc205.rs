use std::collections::HashMap;

impl Solution {
	pub fn is_isomorphic(s: String, t: String) -> bool {
		let s = s.into_bytes();
		let t = t.into_bytes();
		if s.len() != t.len() {
			return false;
		}
		let mut a = HashMap::new();
		let mut b = HashMap::new();
		for i in 0..s.len() {
			if let Some(&c) = a.get(&s[i]) {
				if t[i] != c {
					return false;
				}
			}
			if let Some(&c) = b.get(&t[i]) {
				if s[i] != c {
					return false;
				}
			}
			a.insert(s[i], t[i]);
			b.insert(t[i], s[i]);
		}
		true
	}
}