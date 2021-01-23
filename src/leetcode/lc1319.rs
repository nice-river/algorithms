struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
		let mut ds = DisjointSet::new(n as usize);
		for conn in connections.iter() {
			ds.union(conn[0], conn[1]);
		}
		let tot = connections.len() as i32;
		if tot >= n - 1 {
			ds.get_disjoint_num() as i32 - 1
		} else {
			-1
		}
	}
}

#[derive(Debug, Clone)]
struct DisjointSet {
	marks: Vec<i32>,
	size: Vec<i32>,
}

impl DisjointSet {
	fn new(n: usize) -> Self {
		Self {
			marks: vec![-1; n],
			size: vec![1; n],
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
			self.size[u as usize] += self.size[v as usize];
		}
	}

	fn get_disjoint_num(&mut self) -> usize {
		let mut set = std::collections::HashSet::new();
		for i in 0..self.marks.len() {
			set.insert(self.find(i as i32));
		}
		set.len()
	}

	fn get_size(&mut self, x: i32) -> i32 {
		let x = self.find(x);
		self.size[x as usize]
	}
}

