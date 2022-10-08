struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::BinaryHeap;
use std::iter::FromIterator;

impl Solution {
	pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
		let mut heap = BinaryHeap::from_iter(stones.into_iter());
		while heap.len() > 1 {
			let x = heap.pop().unwrap();
			let y = heap.pop().unwrap();
			if x != y {
				heap.push(x - y);
			}
		}
		heap.pop().unwrap_or(0)
	}
}