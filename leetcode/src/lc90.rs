struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::HashMap;

impl Solution {
	pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		let mut map = HashMap::new();
		for num in nums {
			*map.entry(num).or_insert(0) += 1;
		}

		ans.push(vec![]);
		for (num, cnt) in map.into_iter() {
			for mut x in ans.clone() {
				for _ in 1..=cnt {
					x.push(num);
					ans.push(x.clone());
				}
			}
		}
		ans
	}
}