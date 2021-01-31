struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let strs = ["tars","rats","arts","star"];
		let strs = strs.to_vec().into_iter().map(|s| s.to_string()).collect();
		assert_eq!(Solution::num_similar_groups(strs), 2);
	}
}

impl Solution {
	pub fn num_similar_groups(strs: Vec<String>) -> i32 {
		let n = strs.len();
		let strs: Vec<Vec<u8>> = strs.into_iter().map(|s| s.into_bytes()).collect();
		let mut ds = DisjointSet::new(n);
		for i in 0..n {
			for j in i+1..n {
				let mut cnt = 0;
				for k in 0..strs[i].len() {
					if strs[i][k] != strs[j][k] {
						cnt += 1;
					}
					if cnt > 2 {
						break;
					}
				}
				if cnt == 2 || cnt == 0 {
					ds.union(i, j);
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
