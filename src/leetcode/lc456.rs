struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn find132pattern(nums: Vec<i32>) -> bool {
		if nums.len() < 3 {
			return false;
		}
		let mut stk = vec![*nums.last().unwrap()];
		let mut maxi = std::i32::MIN;

		for i in (0..nums.len()-1).rev() {
			if nums[i] < maxi {
				return true;
			}
			while !stk.is_empty() && &nums[i] > stk.last().unwrap() {
				maxi = maxi.max(stk.pop().unwrap());
			}
			stk.push(nums[i]);
		}
		false
	}
}