struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test() {
		let grid = vec![[1,1,1],[0,1,0],[0,0,0]];
		let hits = vec![[0,2],[2,0],[0,1],[1,2]];
		let grid= grid.into_iter().map(|v| v.to_vec()).collect();
		let hits= hits.into_iter().map(|v| v.to_vec()).collect();
		assert_eq!(Solution::hit_bricks(grid, hits), vec![0, 0, 1, 0]);
	}
}

impl Solution {
	pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
		let n = grid.len();
		let m = grid[0].len();
		let mut status = grid.clone();
		let nil = n * m;
		let mut ds = DisjointSet::new(nil + 1);
		for h in &hits {
			status[h[0] as usize][h[1] as usize] = 0;
		}
		for i in 0..n {
			for j in 0..m {
				if status[i][j] == 0 {
					continue;
				}
				if i == 0 {
					ds.union(nil as i32, j as i32);
				}
				if i > 0 && status[i - 1][j] == 1 {
					ds.union(((i - 1) * m + j) as i32, (i * m + j) as i32);
				}
				if j > 0 && status[i][j - 1] == 1 {
					ds.union((i * m + j - 1) as i32, (i * m + j) as i32);
				}
			}
		}
		let mut prev_sz = ds.get_size(nil as i32);
		let mut ans = vec![];
		let directions = vec![-1, 0, 1, 0, -1];

		for h in hits.iter().rev() {
			let r = h[0];
			let c = h[1];
			if grid[r as usize][c as usize] == 0 {
				ans.push(0);
				continue;
			}
			if r == 0 {
				ds.union(nil as i32, c);
			}
			for d in 0..4 {
				let x = r + directions[d];
				let y = c + directions[d + 1];
				if x < 0 || x >= n as i32 || y < 0 || y >= m as i32 || status[x as usize][y as usize] == 0 {
					continue;
				}
				let a = r as usize * m + c as usize;
				let b = x as usize * m + y as usize;
				ds.union(a as i32, b as i32);
			}
			status[r as usize][c as usize] = 1;
			let cur_sz = ds.get_size(nil as i32);
			ans.push(std::cmp::max(0,  cur_sz - prev_sz - 1));
			prev_sz = cur_sz;
		}

		ans.reverse();
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
