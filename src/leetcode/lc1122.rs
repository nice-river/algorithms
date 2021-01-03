#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}
use std::collections::HashMap;

impl Solution {
	pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
		let mut pos = HashMap::new();
		for (i, &v) in arr2.iter().enumerate() {
			pos.insert(v, i);
		}
		let mut ans = arr1.clone();
		ans.sort_by_key(|v| {
			*pos.get(v).unwrap_or(&((*v) as usize + arr2.len()))
		});
		ans
	}
}