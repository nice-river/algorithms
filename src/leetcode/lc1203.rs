struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = 5;
		let m = 3;
		let group = vec![0, 0, 2, 1, 0];
		let before_items = vec![vec![3],vec![],vec![],vec![],vec![1,3,2]];
		println!("{:?}", Solution::sort_items(n, m, group, before_items));
	}
}

use std::collections::{VecDeque, HashSet};
use std::iter::FromIterator;

impl Solution {
	pub fn sort_items(n: i32, mut m: i32, mut group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
		for g in group.iter_mut() {
			if g == &-1 {
				*g = m;
				m += 1;
			}
		}
		let n = n as usize;
		let m = m as usize;
		let mut inner_gph = vec![vec![]; n];
		let mut outer_gph = vec![vec![]; m];

		let mut inner_in_degree = vec![0; n];
		let mut outer_in_degree = vec![0; m];

		let mut inner_nodes = vec![HashSet::new(); m];

		for (i, before) in before_items.iter().enumerate() {
			inner_nodes[group[i] as usize].insert(i);
			for &b in before {
				if group[b as usize] == group[i] {
					inner_gph[b as usize].push(i);
					inner_in_degree[i] += 1;
				} else {
					outer_gph[group[b as usize] as usize].push(group[i] as usize);
					outer_in_degree[group[i] as usize] += 1;
				}
			}
		}
		let mut ans = vec![];
		let outer_arr = Solution::topo_order(&outer_gph, &mut outer_in_degree, &HashSet::from_iter(0..m));
		for idx in outer_arr {
			let inner_arr = Solution::topo_order(&inner_gph, &mut inner_in_degree, &inner_nodes[idx]);
			ans.extend(inner_arr.iter().map(|&x| x as i32));
		}
		if ans.len() != n {
			vec![]
		} else {
			ans
		}
	}

	fn topo_order(gph: &Vec<Vec<usize>>, in_degree: &mut Vec<usize>, nodes: &HashSet<usize>) -> Vec<usize> {
		let mut que = VecDeque::new();
		for &node in nodes {
			if in_degree[node] == 0 {
				que.push_back(node);
			}
		}
		let mut ret = vec![];
		while !que.is_empty() {
			let node = que.pop_front().unwrap();
			ret.push(node);
			for &nxt in &gph[node] {
				if nodes.contains(&nxt) {
					in_degree[nxt] -= 1;
					if in_degree[nxt] == 0 {
						que.push_back(nxt);
					}
				}
			}
		}
		ret
	}
}