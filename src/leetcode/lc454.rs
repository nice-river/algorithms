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
	pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
		let mut s = HashMap::new();
		let mut t = HashMap::new();
		for x in a {
			for &y in &b {
				*s.entry(x + y).or_insert(0) += 1;
			}
		}
		for x in c {
			for &y in &d {
				*t.entry(x + y).or_insert(0) += 1;
			}
		}
		let mut ans = 0;
		for (&a, &b) in s.iter() {
			if let Some(&d) = t.get(&-a) {
				ans += b * d;
			}
		}
		ans
	}
}