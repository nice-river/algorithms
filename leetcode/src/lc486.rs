use std::cmp::max;

struct Args {
	su: Vec<i32>,
	dp: Vec<Vec<i32>>,
}

impl Solution {
	fn new(nums: &Vec<i32>) -> Args {
		let n = nums.len();
		let mut su = vec![0; n + 1];
		for i in 0..n {
			su[i + 1] = su[i] + nums[i];
		}
		let mut dp = vec![vec![-1; n]; n];
		Args {
			su,
			dp
		}
	}

	pub fn predict_the_winner(nums: Vec<i32>) -> bool {
		let mut sol = Solution::new(&nums);
		let a = Solution::dfs(&mut sol, 0, nums.len() - 1, &nums);
		let tot = sol.su.last().unwrap();
		a >= tot - a
	}

	fn dfs(args: &mut Args, l: usize, r: usize, nums: &Vec<i32>) -> i32 {
		if l == r {
			nums[l]
		} else if args.dp[l][r] != -1 {
			args.dp[l][r]
		} else {
			let a = nums[l] + args.su[r+1] - args.su[l+1] - Solution::dfs(args, l + 1, r, nums);
			let b = nums[r] + args.su[r] - args.su[l] - Solution::dfs(args, l, r - 1, nums);
			args.dp[l][r] = max(a, b);
			args.dp[l][r]
		}
	}
}

struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![1, 5, 2];
		println!("{}", Solution::predict_the_winner(nums));
	}
}