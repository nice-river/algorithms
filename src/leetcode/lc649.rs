struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "RRDDDD".to_string();
		println!("{}", Solution::predict_party_victory(s));
	}
}

use std::collections::HashSet;

impl Solution {
	pub fn predict_party_victory(senate: String) -> String {
		let mut senate = senate.into_bytes();
		let mut tmp = Vec::with_capacity(senate.len());
		let mut ban = vec![0; 2];
		let mut ss = HashSet::new();
		loop {
			for &s in senate.iter() {
				if s == b'R' {
					if ban[0] == 0 {
						ban[1] += 1;
						tmp.push(s);
						ss.insert(s);
					} else {
						ban[0] -= 1;
					}
				} else {
					if ban[1] == 0 {
						ban[0] += 1;
						tmp.push(s);
						ss.insert(s);
					} else {
						ban[1] -= 1;
					}
				}
			}
			if ss.len() == 1 {
				break;
			}
			ss.clear();
			senate = tmp.clone();
			tmp.clear();
		}
		if ss.iter().next().unwrap() == &b'R' {
			"Radiant".to_string()
		} else {
			"Dire".to_string()
		}
	}
}