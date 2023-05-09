struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let graph = vec![vec![1,2,3],vec![0],vec![0],vec![0]];
		let ans = 4;
		assert_eq!(Solution::shortest_path_length(graph), ans);
	}

	#[test]
	fn test1() {
		let graph = vec![vec![1],vec![0,2,4],vec![1,3,4],vec![2],vec![1,2]];
		let ans = 4;
		assert_eq!(Solution::shortest_path_length(graph), ans);
	}
}

use std::collections::VecDeque;

impl Solution {
	pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
		let n = graph.len();
		let mut dp = vec![vec![i32::MAX; 1 << n]; n];
		let mut que = VecDeque::new();
		let all = (1 << n) - 1;
		for i in 0..n {
			dp[i][all ^ (1 << i)] = 0;
			que.push_back((i, all ^ (1 << i)));
		}
		while !que.is_empty() {
			let (x, y) = que.pop_front().unwrap();
			let step = dp[x][y];
			if y == 0 {
				return step;
			}
			for &nxt in graph[x].iter() {
				let nxt = nxt as usize;
				let g = y & (all ^ (1 << nxt));
				if step + 1 < dp[nxt][g] {
					dp[nxt][g] = step + 1;
					que.push_back((nxt, g));
				}
			}
		}
		-1
	}
}