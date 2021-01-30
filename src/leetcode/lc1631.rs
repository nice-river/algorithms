struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let heights = [[8,6,8,1,4,1],[10,3,1,8,9,10],[1,5,6,9,8,5],[10,4,6,7,3,3],[6,6,9,1,3,3],[3,1,10,3,4,1],[6,1,6,10,7,10]];
		let ans = 3;
		let heights = heights.to_vec().into_iter().map(|v| v.to_vec()).collect();
		assert_eq!(Solution::minimum_effort_path(heights), ans);
	}
}

use std::collections::VecDeque;

impl Solution {
	pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
		let n = heights.len();
		let m = heights[0].len();
		let mut dist = vec![vec![i32::max_value(); m]; n];
		let mut que = VecDeque::new();
		que.push_back((0, 0));
		dist[0][0] = 0;
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
				let diff = (heights[cur.0][cur.1] - heights[x][y]).abs();
				let v = std::cmp::max(dist[cur.0][cur.1], diff);
				if dist[x][y] > v {
					dist[x][y] = v;
					que.push_back((x as i32, y as i32));
				}
			}
		}
		dist[n-1][m-1]
	}
}