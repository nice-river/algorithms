use std::convert::TryInto;

struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let is_connected = vec![[1,1,0],[1,1,0],[0,0,1]];
		let is_connected: Vec<Vec<i32>> = is_connected.into_iter().map(|v| v.to_vec()).collect();
		assert_eq!(Solution::find_circle_num(is_connected), 2);
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


impl Solution {
	pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
		let n = is_connected.len();
		let mut ds = DisjointSet::new(n);
		for i in 0..n {
			for j in 0..n {
				if i != j && is_connected[i][j] == 1 {
					ds.union(i as i32, j as i32);
				}
			}
		}
		ds.get_disjoint_num() as i32
	}
}