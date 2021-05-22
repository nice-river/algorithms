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
	pub fn xor_game(nums: Vec<i32>) -> bool {
        nums.len() % 2 == 0 || nums.into_iter().fold(0, |a, b| a ^ b) == 0
	}
}
