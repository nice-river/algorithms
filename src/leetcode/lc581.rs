struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![2,6,4,8,10,11,1];
		let ans = 7;
		assert_eq!(Solution::find_unsorted_subarray(nums), ans);
	}
}


impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
		if nums.len() <= 1 { return 0; }
		let mut left = 1;
		let mut ans = 0;
		let mut cur_max = nums[0];
		for i in 1..nums.len() {
			let num = nums[i];
			if num < cur_max {
				while left >= 1 && nums[left -1] > num {
					left -= 1;
				}
				ans = i - left + 1;
			} else {
				cur_max = num;
				if left == i {
					left += 1;
				}
			}
		}
		ans as i32
    }
}