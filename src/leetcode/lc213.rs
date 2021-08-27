struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![2, 3, 2];
        assert_eq!(Solution::rob(nums), 3);

		let nums = vec![2, 3, 4];
		assert_eq!(Solution::rob(nums), 4);

		let nums = vec![4, 3, 2];
		assert_eq!(Solution::rob(nums), 4);

	}
}



impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
			return *nums.last().unwrap_or(&0);
		}
        Solution::dp(&nums[0..nums.len()-1]).max(Solution::dp(&nums[1..nums.len()]))
    }

	fn dp(nums: &[i32]) -> i32 {
		let (mut a, mut b) = (0, 0);
		for &num in nums {
            let x = b + num;
			let y = a.max(b);
			a = x;
			b = y;
		}
		a.max(b)
	}
}