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
	pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
		let target = target as usize;
		let mut dp = vec![-1; target + 1];
        Solution::dfs(target, &nums, &mut dp)
	}

	fn dfs(target: usize, nums: &[i32], dp: &mut Vec<i32>) -> i32 {
        if target == 0 {
			return 1;
		}
		if dp[target] != -1 {
			return dp[target];
		}
		let mut ret = 0;
		for &num in nums {
            if let Some(x) = target.checked_sub(num as usize) {
                ret += Solution::dfs(x, nums, dp);
			}
		}
		dp[target] = ret;
		ret
	}
}