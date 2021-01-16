struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let edges = vec![[2,7],[7,8],[3,6],[2,5],[6,8],[4,8],[2,8],[1,8],[7,10],[3,9]];
		let edges = edges.into_iter().map(|a| a.to_vec()).collect();
		assert_eq!(Solution::find_redundant_connection(edges), vec![2,8])
	}
}

impl Solution {
	pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
		let n = edges.len();
		let mut gph = vec![vec![]; n + 1];
		for (idx, edge) in edges.iter().enumerate() {
			gph[edge[0] as usize].push((edge[1] as usize, idx));
			gph[edge[1] as usize].push((edge[0] as usize, idx));
		}
		let mut vis = vec![false; n + 1];
		let mut paths = vec![];
		let mut path_vis = vec![false; edges.len()];
		let mut start_idx = vec![0; n + 1];
		if let Some(path_idx) = Solution::dfs(1, &mut vis, &gph, &mut paths, &mut path_vis, &mut start_idx) {
			let edge = &edges[path_idx];
			vec![std::cmp::min(edge[0], edge[1]), std::cmp::max(edge[0], edge[1])]
		} else {
			vec![]
		}
	}

	fn dfs(idx: usize, vis: &mut Vec<bool>, gph: &Vec<Vec<(usize, usize)>>, paths: &mut Vec<usize>, path_vis: &mut Vec<bool>, start_idx: &mut Vec<usize>) -> Option<usize> {
		if vis[idx] {
			return Some(paths.drain(start_idx[idx]..paths.len()).max().unwrap());
		}
		vis[idx] = true;
		start_idx[idx] = paths.len();
		for &(nxt, path_idx) in &gph[idx] {
			if !path_vis[path_idx] {
				path_vis[path_idx] = true;
				paths.push(path_idx);
				let ret = Solution::dfs(nxt, vis, gph, paths, path_vis, start_idx);
				if ret.is_some() {
					return ret;
				}
				paths.pop();
				path_vis[path_idx] = false;
			}
		}
		None
	}
}