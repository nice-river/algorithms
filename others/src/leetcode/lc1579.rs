struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = 4;
		let edges = [[3,1,2],[3,2,3],[1,1,3],[1,2,4],[1,1,2],[2,3,4]];
		let edges = edges.to_vec().into_iter().map(|v| v.to_vec()).collect();
		assert_eq!(Solution::max_num_edges_to_remove(n, edges), 2);
	}
}

use std::collections::HashSet;

impl Solution {
	pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
		let n = n as usize;
		edges.sort_unstable_by_key(|v| {
			if v[0] == 3 { 0 } else { 1 }
		});
		let mut dss = [DisjointSet::new(n), DisjointSet::new(n)];
		let mut sets = [HashSet::new(), HashSet::new()];

		for t in 0..=1 {
			for i in 0..edges.len() {
				if edges[i][0] == 3 || edges[i][0] as usize - 1 == t {
					let u = dss[t].find(edges[i][1] - 1);
					let v = dss[t].find(edges[i][2] - 1);
					if u != v {
						dss[t].union(u, v);
						sets[t].insert(i);
					}
				}
			}
			if dss[t].get_disjoint_num() != 1 {
				return -1;
			}
		}
		(edges.len() - sets[0].union(&sets[1]).collect::<HashSet<_>>().len()) as i32
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
