#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		// let edges = vec![vec![1,5], vec![3,2], vec![2,4], vec![4,5], vec![5,3]];
		let edges = vec![vec![6,3],vec![8,4],vec![9,6],vec![3,2],vec![5,10],vec![10,7],vec![2,1],vec![7,6],vec![4,5],vec![1,8]];
		let ret = Solution::find_redundant_directed_connection(edges);

		assert_eq!(ret, vec![7, 6]);
	}
}

struct Solution {}

use std::collections::{HashMap, HashSet};
use std::cmp::max;


impl Solution {
	pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
		let n: usize = edges.len();
		let mut vis = vec![false; n + 1];
		let mut gph = vec![vec![]; n + 1];
		let mut parents = HashMap::new();
		let mut path = vec![];
		let mut paths = HashSet::new();

		for (idx, edge) in edges.iter().enumerate() {
			gph[edge[0] as usize].push((edge[1] as usize, idx));
		}

		let mut max_circle_idx = n;
		for i in 1..=n {
			if !vis[i] {
				vis[i] = true;
				parents.insert(i, 0);
				path.push(i);
				let ret = Solution::dfs(i, &gph, &mut parents, &mut vis, &mut path, &mut paths);
				if let Some(idx) = ret {
					max_circle_idx = idx;
					break;
				}
				parents.remove(&i);
				path.pop();
			}
		}

		let mut parent = vec![0; n + 1];

		for (idx, edge) in edges.iter().enumerate() {
			if parent[edge[1] as usize] != 0{
				if paths.contains(&(parent[edge[1] as usize], edge[1] as usize)) {
					return vec![parent[edge[1] as usize] as i32, edge[1]];
				}
				return edge.clone();
			}
			parent[edge[1] as usize] = edge[0] as usize;
		}

		edges[max_circle_idx].clone()
	}

	fn dfs(cur: usize, gph: &Vec<Vec<(usize, usize)>>, parents: &mut HashMap<usize, usize>, vis: &mut Vec<bool>, path: &mut Vec<usize>, paths: &mut HashSet<(usize, usize)>) -> Option<usize>{
		for (nxt, idx) in &gph[cur] {
			if parents.contains_key(&nxt) {
				let mut start_idx = 0;
				for (i, p) in path.iter().enumerate() {
					if p == nxt {
						start_idx = i;
						break;
					}
				}
				let mut ret = *idx;
				for i in start_idx+1..path.len() {
					ret = max(ret, parents[&path[i]]);
					paths.insert((path[i-1], path[i]));
				}
				paths.insert((cur, *nxt));
				return Some(ret);
			}
			if !vis[*nxt] {
				vis[*nxt] = true;
				parents.insert(*nxt, *idx);
				path.push(*nxt);
				let ret = Solution::dfs(*nxt, gph, parents, vis, path, paths);
				if ret.is_some() {
					return ret;
				}
				parents.remove(nxt);
				path.pop();
			}
		}
		None
	}
}
