struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sol = Solution {};
	}
}

impl Solution {
	pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		let mut sol = vec![];
		if k > 9 || n > 45 {
			return ans;
		}
		Solution::dfs(0, n, k, &mut sol, &mut ans);
		ans
	}

	fn dfs(su: i32, n: i32, k: i32, sol: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
		if sol.len() == k as usize {
			if su == n {
				ans.push(sol.clone());
			}
			return ;
		}
		let last = (*sol.last().unwrap_or(&0) + 1) as usize;

		if 9 - last + 1 < k as usize - sol.len() {
			return ;
		}

		for i in last..=9 {
			sol.push(i as i32);
			Solution::dfs(su + i as i32, n, k, sol, ans);
			sol.pop();
		}
	}
}
