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
	pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
		let mut map = HashMap::new();
		let mut ans = 0;
		for i in 0..dominoes.len() {
			let t: (i32, i32) = (std::cmp::min(dominoes[i][0], dominoes[i][1]), std::cmp::max(dominoes[i][0], dominoes[i][1]));
			ans += *map.get(&t).unwrap_or(&0);
			*map.entry(t).or_insert(0) += 1;
		}
		ans
	}
}