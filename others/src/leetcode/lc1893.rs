use std::vec;

struct Solution {}

#[cfg(test)]
mod tests {
	use std::vec;

use super::*;

	#[test]
	fn test() {
		let ranges = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
		let left = 2;
		let right = 5;
		assert!(Solution::is_covered(ranges, left, right))
	}
}


impl Solution {
	pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
		let mut arr = vec![0; 52];
		for range in ranges {
			arr[range[0] as usize] += 1;
			arr[range[1] as usize + 1] -= 1;
		}
		let mut s = 0;
		for i in 1..=50 {
			s += arr[i as usize];
			if left <= i && i <= right && s <= 0 {
				return false;
			}
		}
		true
	}
}