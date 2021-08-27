struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		let mut path = vec![];
		Solution::dfs(0, &graph, &mut path, &mut ans);
		ans
	}

	fn dfs(cur: usize, graph: &Vec<Vec<i32>>, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
		let n = graph.len();
		path.push(cur as i32);
		if cur == n - 1 {
			ans.push(path.clone());
		} else {
			for &nxt in graph[cur].iter() {
				Solution::dfs(nxt as usize, graph, path, ans);
			}
		}
		path.pop();
	}
}