struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sol = Solution {};
	}
}

use std::collections::{HashMap, BTreeMap};

impl Solution {
	pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
		let mut counter = HashMap::new();
		for num in nums {
			*counter.entry(num).or_insert(0) += 10;
		}
		let mut bst = BTreeMap::new();
		for (k, v) in counter.into_iter() {
			bst.entry(v).or_insert(Vec::new()).push(k);
		}
		let mut ans = vec![];
		for (_, v) in bst.iter().rev() {
			for num in v.iter() {
				ans.push(*num);
			}
			if ans.len() == k as usize {
				break;
			}
		}
		ans
	}
}

