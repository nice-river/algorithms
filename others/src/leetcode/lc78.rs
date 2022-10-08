#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
	pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		let mut cur = vec![];

		for i in 0..=nums.len() {
			Solution::dfs(0, i, &nums, &mut cur, &mut ans);
		}

		return ans;
	}

	fn dfs(idx: usize, n: usize, nums: &Vec<i32>, cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
		if cur.len() == n {
			ans.push(cur.clone());
			return;
		}
		if idx == nums.len() {
			return ;
		}
		for i in idx..nums.len() {
			cur.push(nums[i]);
			Solution::dfs(i + 1, n, nums, cur, ans);
			cur.pop();
		}
	}
}
