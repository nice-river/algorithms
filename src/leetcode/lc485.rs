struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
		let mut pre = -1;
		let mut ans = 0;
		for (i, &num) in nums.iter().enumerate() {
			if num == 0 {
				ans = ans.max(i as i32 - pre - 1);
				pre = i as i32;
			}
		}
		ans = ans.max(nums.len() as i32 - pre - 1);
		ans
	}
}