struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
		let mut ans = vec![None; graph.len()];
		for i in 0..graph.len() {
			Solution::dfs(i, &graph, &mut ans);
		}
		ans.into_iter().enumerate().filter_map(|(idx, x)| {
			if x.unwrap() {
				Some(idx as i32)
			} else {
				None
			}
		}).collect()
	}

	fn dfs(idx: usize, graph: &Vec<Vec<i32>>, ans: &mut Vec<Option<bool>>) -> bool {
		if let Some(x) = ans[idx] {
			return x;
		}
		if graph[idx].len() == 0 {
			ans[idx] = Some(true);
			return true;
		}
		ans[idx] = Some(false);
		for &nxt in graph[idx].iter() {
			let ret = Solution::dfs(nxt as usize, graph, ans);
			if !ret {
				return false;
			}
		}
		ans[idx] = Some(true);
		return true;
	}
}