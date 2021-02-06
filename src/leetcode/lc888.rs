struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
	pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
		let p: i32 = a.iter().sum();
		let q: i32 = b.iter().sum();
		let s: HashSet<_> = HashSet::from_iter(b);
		let mut ans = vec![0, 0];
		for &x in a.iter() {
			let y = q - p + 2 * x;
			if y % 2 == 0 && s.contains(&(y / 2)) {
				ans[0] = x;
				ans[1] = y / 2;
			}
		}
		ans
	}
}