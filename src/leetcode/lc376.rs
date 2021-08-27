struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
		let mut nums= nums.clone();
		nums.dedup();
		if nums.len() <= 2 {
			return nums.len() as i32;
		}
		let mut stk = vec![nums[0], nums[1]];
		for i in 2..nums.len() {
			let a = stk[stk.len() - 1];
			let b = stk[stk.len() - 2];
			if a - b > 0 {
				if nums[i] >= a {
					stk.pop();
				}
			} else {
				if nums[i] <= a {
					stk.pop();
				}
			}
			stk.push(nums[i]);
		}
		stk.len() as i32
	}
}