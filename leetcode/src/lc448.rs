struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![4,3,2,7,8,2,3,1];
		assert_eq!(Solution::find_disappeared_numbers(nums), vec![5, 6]);
	}
}

impl Solution {
	pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
		let n = nums.len() as i32;
		for i in 0..nums.len() {
			let x = (nums[i] - 1) % n;
			nums[x as usize] += n;
		}
		let mut ans = vec![];
		for (i, num) in nums.into_iter().enumerate() {
			if num <= n {
				ans.push(i as i32 + 1);
			}
		}
		ans
	}
}