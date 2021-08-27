struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let edges = vec![vec![0,1],vec![0,2],vec![2,3],vec![2,4],vec![2,5]];
		let ans = Solution::sum_of_distances_in_tree(6, edges);
		assert_eq!(ans, vec![8, 12, 6, 10, 10, 10]);
	}
}

impl Solution {
	pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
		let mut sub_cnt = vec![0; n as usize];
		let mut ans = vec![0; n as usize];
		let mut gph = vec![vec![]; n as usize];
		let mut vis = vec![false; n as usize];
		for edge in edges.iter() {
			gph[edge[0] as usize].push(edge[1] as usize);
			gph[edge[1] as usize].push(edge[0] as usize);
		}
		vis[0] = true;
		Solution::dfs0(0, &gph, &mut vis, &mut sub_cnt, &mut ans);
		vis.iter_mut().map(|e| *e = false).count();
		vis[0] = true;
		Solution::dfs1(0, 0, n, &gph, &mut vis, &mut sub_cnt, &mut ans);
		ans
	}

	fn dfs0(root: usize, gph: &Vec<Vec<usize>>, vis: &mut Vec<bool>, sub_cnt: &mut Vec<i32>, ans: &mut Vec<i32>) {
		sub_cnt[root] = 1;
		for &ch in gph[root as usize].iter() {
			if !vis[ch] {
				vis[ch] = true;
				Solution::dfs0(ch, gph, vis, sub_cnt, ans);
				sub_cnt[root] += sub_cnt[ch];
				ans[root] += ans[ch] + sub_cnt[ch];
			}
		}
	}

	fn dfs1(root: usize, parent: usize, n: i32, gph: &Vec<Vec<usize>>, vis: &mut Vec<bool>, sub_cnt: &mut Vec<i32>, ans: &mut Vec<i32>) {
		if parent != root {
			let a = ans[parent] - ans[root] - sub_cnt[root];
			let b = n - sub_cnt[root];
			ans[root] += a + b;
		}
		for &ch in gph[root as usize].iter() {
			if !vis[ch] {
				vis[ch] = true;
				Solution::dfs1(ch, root, n, gph, vis, sub_cnt, ans);
			}
		}
	}
}