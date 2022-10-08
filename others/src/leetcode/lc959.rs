struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let grid = vec!["/\\","\\/"];
		let grid = grid.into_iter().map(|e| e.to_string()).collect();
		let ans = 5;
		assert_eq!(Solution::regions_by_slashes(grid), ans);
	}
}

impl Solution {
	pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
		let n = grid.len();
		let m = grid[0].as_bytes().len();
		let mut ds = DisjointSet::new(n * m * 4);
		for (i, row) in grid.iter().enumerate() {
			for (j, &c) in row.as_bytes().iter().enumerate() {
				let k = (i * m + j) * 4;
				if c == b'/' {
					ds.union(k, k + 3);
					ds.union(k + 1, k + 2);
				} else if c == b'\\' {
					ds.union(k, k + 1);
					ds.union(k + 2, k + 3);
				} else {
					ds.union(k, k + 1);
					ds.union(k + 1, k + 2);
					ds.union(k + 2, k + 3);
				}
				if i > 0 {
					ds.union(k, k - m * 4 + 2);
				}
				if j > 0 {
					ds.union(k + 3, k - 4 + 1);
				}
			}
		}
		ds.get_disjoint_num() as i32
	}
}

#[derive(Debug, Clone)]
struct DisjointSet {
	marks: Vec<i32>,
	size: Vec<i32>,
	count: usize,
}

impl DisjointSet {
	fn new(n: usize) -> Self {
		Self {
			marks: vec![-1; n],
			size: vec![1; n],
			count: n
		}
	}

	fn find<T: std::convert::TryInto<usize>>(&mut self, x: T) -> i32 {
		let ux = x.try_into().ok().unwrap();
		if self.marks[ux] != -1 {
			self.marks[ux] = self.find(self.marks[ux]);
			self.marks[ux]
		} else {
			ux as i32
		}
	}

	fn union<T: std::convert::TryInto<usize>, S: std::convert::TryInto<usize>>(&mut self, u: T, v: S) {
		let u = self.find(u);
		let v = self.find(v);
		if u != v {
			self.marks[v as usize] = u;
			self.size[u as usize] += self.size[v as usize];
			self.count -= 1;
		}
	}

	fn get_disjoint_num(&mut self) -> usize {
		self.count
	}

	fn get_size<T: std::convert::TryInto<usize>>(&mut self, x: T) -> i32 {
		let x = self.find(x);
		self.size[x as usize]
	}
}
