struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let points = [[2,-3],[-17,-8],[13,8],[-17,-15]];
		let points = points.to_vec().into_iter().map(|v| v.to_vec()).collect();
		assert_eq!(Solution::min_cost_connect_points(points), 53);
	}
}

impl Solution {
	pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
		let n = points.len();
		let mut arr = Vec::with_capacity(n * n);
		for i in 0..n {
			for j in i+1..n {
				let k = (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
				arr.push((k, i, j));
			}
		}
		arr.sort_unstable();
		let mut ans = 0;
		let mut ds = DisjointSet::new(n);
		for i in 0..arr.len() {
			let top = arr[i];
			if ds.find(top.1 as i32) != ds.find(top.2 as i32) {
				ans += top.0;
				ds.union(top.1 as i32,  top.2 as i32);
			}
		}
		ans
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
