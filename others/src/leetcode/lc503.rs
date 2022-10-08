struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = [1, 2, 1];
		let ans = [2, -1, 2];
		assert_eq!(Solution::next_greater_elements(nums.to_vec()), ans.to_vec());
	}

	#[test]
	fn test1() {
		let nums = [3, 2, 1];
		let ans = [-1, 3, 3];
		assert_eq!(Solution::next_greater_elements(nums.to_vec()), ans.to_vec());
	}

	#[test]
	fn test2() {
		let nums = [1, 2, 3];
		let ans = [2, 3, -1];
		assert_eq!(Solution::next_greater_elements(nums.to_vec()), ans.to_vec());
	}

	#[test]
	fn test3() {
		let nums = [1];
		let ans = [-1];
		assert_eq!(Solution::next_greater_elements(nums.to_vec()), ans.to_vec());
	}
}

impl Solution {
	pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
		let n = nums.len();
		let mut ans = vec![-1; n];
		let mut stk = vec![];
		for i in 0..2 * n {
			while let Some(&e) = stk.last() {
				if nums[e % n] < nums[i % n] {
					ans[e % n] = nums[(i % n)];
					stk.pop();
				} else {
					break;
				}
			}
			stk.push(i);
		}
		ans
	}
}