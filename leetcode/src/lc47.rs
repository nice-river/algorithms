#[cfg(test)]
mod tests {
	#[test]
	fn test() {
	}
}

struct Solution {}

use std::collections::HashMap;


impl Solution {
	pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
		let n = nums.len();
		let mut counter = HashMap::new();
		for num in &nums {
			*counter.entry(*num).or_insert(0) += 1;
		}

		let mut candidates = vec![];
		for (k, v) in counter.iter() {
			candidates.push((*k, *v))
		}

		let mut cur = vec![];
		let mut ans = vec![];

		Solution::dfs(&mut candidates, &mut cur, &mut ans, n);

		ans
	}

	fn dfs(candidates: &mut Vec<(i32, i32)>, cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, n: usize) {
		if cur.len() == n {
			ans.push(cur.clone());
			return;
		}
		for i in 0..candidates.len() {
			if candidates[i].1 != 0 {
				cur.push(candidates[i].0);
				candidates[i].1 -= 1;
				Solution::dfs(candidates, cur, ans, n);
				candidates[i].1 += 1;
				cur.pop();
			}
		}

	}
}