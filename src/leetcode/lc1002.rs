struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}
use std::collections::HashMap;
use std::cmp::min;

impl Solution {
	pub fn common_chars(a: Vec<String>) -> Vec<String> {
		if a.len() == 0 {
			return vec![];
		}
		let mut m = HashMap::new();
		for ch in a.first().unwrap().chars() {
			*m.entry(ch).or_insert(0) += 1;
		}
		for i in 1..a.len() {
			let mut x = HashMap::new();
			for ch in a[i].chars() {
				*x.entry(ch).or_insert(0) += 1;
			}
			let mut del = vec![];
			{
				for (k, v) in m.iter() {
					let t = x.get(k);
					if let Some(&val) = t {
						del.push((k.clone(), min(v.clone(), val)));
					} else {
						del.push((k.clone(), 0));
					}
				}
			}
			for (k, v) in del {
				if v == 0 {
					m.remove(&k);
				} else {
					m.insert(k, v);
				}
			}
		}
		let mut ans = vec![];
		for (k, v) in m.into_iter() {
			for _ in 0..v as usize {
				ans.push(k.to_string());
			}
		}
		ans
	}
}