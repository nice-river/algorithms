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
	pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		if k == 0 || n < k {
			return ans;
		}
		Solution::dfs(0, n, k, &mut vec![], &mut ans);
		ans
	}

	fn dfs(num: i32, n: i32, k: i32, cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
		if cur.len() == k as usize {
			ans.push(cur.clone());
			return
		}
		if n - num < k - cur.len() as i32 {
			return
		}
		for i in num+1..=n {
			cur.push(i);
			Solution::dfs(i, n, k, cur, ans);
			cur.pop();
		}
	}
}
