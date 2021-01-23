use std::cmp::min;

struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = 2;
		let edges = [[0,1,3]];
		let edges = edges.to_vec().into_iter().map(|v| v.to_vec()).collect();
		assert_eq!(Solution::find_critical_and_pseudo_critical_edges(n, edges), vec![vec![0], vec![]]);
	}
}

impl Solution {
	pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let n = n as usize;
		let mut edges: Vec<Vec<i32>> = edges.into_iter().enumerate().map(|(i, mut v)| {
			v.push(i as i32);
			v
		}).collect();
		edges.sort_unstable_by_key(|v| v[2]);
		let mini = Solution::minimum_span_tree_exclude(n, &edges, edges.len());
		let mut a = vec![];
		let mut b = vec![];
		for i in 0..edges.len() {
			let r = Solution::minimum_span_tree_exclude(n, &edges, i);
			if r != mini {
				a.push(edges[i][3]);
			} else {
				let r = Solution::minimum_span_tree_include(n, &edges, i);
				if r == mini {
					b.push(edges[i][3]);
				}
			}
		}
		vec![a, b]
	}

	fn minimum_span_tree_exclude(n: usize, edges: &Vec<Vec<i32>>, exclude_idx: usize) -> usize {
		let mut ret = 0;
		let mut ds = DisjointSet::new(n);
		for (idx, e) in edges.iter().enumerate() {
			if idx == exclude_idx {
				continue;
			}
			let a = ds.find(e[0]);
			let b = ds.find(e[1]);
			if a != b {
				ds.union(a, b);
				ret += e[2];
			}
		}
		ret as usize
	}

	fn minimum_span_tree_include(n: usize, edges: &Vec<Vec<i32>>, include_idx: usize) -> usize {
		let mut ds = DisjointSet::new(n);
		let mut ret = 0;
		ret += edges[include_idx][2];
		ds.union(edges[include_idx][0], edges[include_idx][1]);
		for (idx, e) in edges.iter().enumerate() {
			if idx == include_idx {
				continue;
			}
			let a = ds.find(e[0]);
			let b = ds.find(e[1]);
			if a != b {
				ds.union(a, b);
				ret += e[2];
			}
		}
		ret as usize
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

