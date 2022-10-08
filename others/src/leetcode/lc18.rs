struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


use std::collections::HashSet;


impl Solution {
	pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
		let mut nums = nums;
		nums.sort();
		let mut ans = vec![];
		let n = nums.len();
		let mut s = HashSet::with_capacity(n);
		for i in 0..n {
			if i > 0 && nums[i] == nums[i - 1] { continue; }
			for j in i + 1..n {
				if j > i + 1 && nums[j] == nums[j - 1] { continue; }
				s.clear();
				let t= target - nums[i] - nums[j];
				let mut found = false;
				for k in j + 1..n {
					if k > j + 1 && nums[k] == nums[k - 1] && found { continue; }
					found = false;
					if s.contains(&(t - nums[k])) {
						ans.push(vec![nums[i], nums[j], nums[k], t - nums[k]]);
						found = true;
					}
					s.insert(nums[k]);
				}
			}
		}
		ans
	}
}