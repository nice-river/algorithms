struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let grid = [[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]];
		let ans = 16;
		let grid = grid.to_vec().into_iter().map(|v| v.to_vec()).collect();
		assert_eq!(Solution::swim_in_water(grid), ans);
	}
}

use std::collections::VecDeque;

impl Solution {
	pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
		let n = grid.len();
		let m = grid[0].len();
		let mut dist = vec![vec![i32::max_value(); m]; n];
		let mut que = VecDeque::new();
		que.push_back((0, 0));
		dist[0][0] = grid[0][0];
		let dirs = [-1, 0, 1, 0, -1];
		while !que.is_empty() {
			let cur = que.pop_front().unwrap();
			for d in 0..4 {
				let x = cur.0 + dirs[d];
				let y = cur.1 + dirs[d + 1];
				if x < 0 || y < 0 || x >= n as i32 || y >= m as i32 {
					continue ;
				}
				let cur = (cur.0 as usize, cur.1 as usize);
				let (x, y) = (x as usize, y as usize);
				let v = std::cmp::max(dist[cur.0][cur.1], grid[x][y]);
				if dist[x][y] > v {
					dist[x][y] = v;
					que.push_back((x as i32, y as i32));
				}
			}
		}
		dist[n-1][m-1]
	}
}