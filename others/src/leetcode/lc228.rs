#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
	pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
		let mut i = 0;
		let mut ans = vec![];
		while i < nums.len() {
			let mut j = i + 1;
			while j < nums.len() && nums[j-1] + 1 == nums[j] {
				j += 1;
			}
			if j == i + 1 {
				ans.push(nums[i].to_string());
			} else {
				ans.push(format!("{}->{}", nums[i], nums[j-1]));
			}
			i = j;
		}
		ans
	}
}