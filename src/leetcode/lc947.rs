struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::HashMap;

impl Solution {
	pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
		let mut rows = HashMap::new();
		let mut cols = HashMap::new();
		for (idx, stone) in stones.iter().enumerate() {
			rows.entry(stone[0]).or_insert(vec![]).push(idx);
			cols.entry(stone[1]).or_insert(vec![]).push(idx);
		}
		let mut vis = vec![false; stones.len()];
		let mut ans = 0;
		for i in 0..stones.len() {
			let mut cnt = 0;
			if !vis[i] {
				Solution::dfs(i, &stones, &mut vis, &rows, &cols, &mut cnt);
				ans += cnt - 1;
			}
		}
		ans
	}

	fn dfs(idx: usize, stones: &Vec<Vec<i32>>, vis: &mut Vec<bool>, rows: &HashMap<i32, Vec<usize>>, cols: &HashMap<i32, Vec<usize>>, cnt: &mut i32) {
		vis[idx] = true;
		*cnt += 1;
		let row = stones[idx][0];
		let col = stones[idx][1];
		for &nxt in rows.get(&row).unwrap() {
			if nxt != idx && !vis[nxt] {
				Solution::dfs(nxt, stones, vis, rows, cols, cnt);
			}
		}
		for &nxt in cols.get(&col).unwrap() {
			if nxt != idx && !vis[nxt] {
				Solution::dfs(nxt, stones, vis, rows, cols, cnt);
			}
		}
	}
}