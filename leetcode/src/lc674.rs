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
	pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
		let mut i = 0;
		let mut ans = 0;
		while i < nums.len() {
			let mut j = i + 1;
			while j < nums.len() && nums[j] > nums[j - 1] {
				j += 1;
			}
			ans = std::cmp::max(ans, (j - i) as i32);
			i = j;
		}
		ans
	}
}