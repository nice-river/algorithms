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
	pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
		let s = s.as_bytes();
		let mut ds = DisjointSet::new(s.len());
		for pair in &pairs {
			ds.union(pair[0], pair[1]);
		}
		let mut parts = HashMap::new();
		for i in 0..s.len() {
			let p = ds.find(i as i32);
			parts.entry(p).or_insert(vec![]).push(s[i]);
		}
		for v in parts.values_mut() {
			v.sort_unstable();
		}
		let mut pos = HashMap::new();
		let mut ans = vec![];
		for i in 0..s.len() {
			let p = ds.find(i as i32);
			let k = pos.entry(p).or_insert(0);
			let v = parts.get(&p).unwrap()[*k];
			*k += 1;
			ans.push(v);
		}
		String::from_utf8(ans).unwrap()
	}
}

struct DisjointSet {
	marks: Vec<i32>,
}

impl DisjointSet {
	fn new(n: usize) -> Self {
		Self {
			marks: vec![-1; n]
		}
	}

	fn find(&mut self, x: i32) -> i32 {
		if self.marks[x as usize] != -1 {
			self.marks[x as usize] = self.find(self.marks[x as usize]);
			self.marks[x as usize]
		} else {
			x
		}
	}

	fn union(&mut self, u: i32, v: i32) {
		let u = self.find(u);
		let v = self.find(v);
		if u != v {
			self.marks[v as usize] = u;
		}
	}

	fn get_disjoint_num(&mut self) -> usize {
		let mut set = std::collections::HashSet::new();
		for i in 0..self.marks.len() {
			set.insert(self.find(i as i32));
		}
		set.len()
	}
}
