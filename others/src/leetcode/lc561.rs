struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![1,4,3,2];
		let ans = 4;
		assert_eq!(Solution::array_pair_sum(nums), ans);

		let nums = vec![6, 2, 6, 5, 1, 2];
		let ans = 9;
		assert_eq!(Solution::array_pair_sum(nums), ans);
	}
}

impl Solution {
	pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
		nums.sort_unstable();
		nums.iter().step_by(2).fold(0, |a, &b| a + b)
	}
}