struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mat = vec![vec![1,1,0,0,0],
			vec![1,1,1,1,0],
			vec![1,0,0,0,0],
			vec![1,1,0,0,0],
			vec![1,1,1,1,1]];
		let k = 3;
		let ans = vec![2, 0, 3];
		assert_eq!(Solution::k_weakest_rows(mat, k), ans);
	}
}


use std::collections::BinaryHeap;

impl Solution {
	pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
		let mut heap = BinaryHeap::new();
		for (idx, row) in mat.iter().enumerate() {
			let cnt = row.iter().position(|&x| x == 0).unwrap_or(row.len());
			heap.push((cnt, idx));
			if heap.len() > k as usize {
				heap.pop();
			}
		}
		let mut ans = vec![];
		while let Some((_, x)) = heap.pop() {
			ans.push(x as i32);
		}
		ans.reverse();
		ans
	}
}