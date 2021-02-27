struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "aaabb".to_string();
		let k = 3;
		assert_eq!(Solution::longest_substring(s, k), 3);

		let s = "ababbc".to_string();
		let k = 2;
		assert_eq!(Solution::longest_substring(s, k), 5);

		let s = "abab".to_string();
		let k = 2;
		assert_eq!(Solution::longest_substring(s, k), 4);

		let s = "abacbb".to_string();
		let k = 2;
		assert_eq!(Solution::longest_substring(s, k), 2);
	}
}

use std::collections::HashSet;

impl Solution {
	pub fn longest_substring(s: String, k: i32) -> i32 {
		let s = s.as_bytes();
		let mut cnts = vec![vec![0; s.len() + 1]; 26];
		for i in 0..26 {
			for (j, &b) in s.iter().enumerate() {
				cnts[i][j + 1] = cnts[i][j] + if b - b'a' == i as u8 {1} else {0}
			}
		}
		Solution::get_substring(s, 0, s.len(), k, &cnts)
	}

	fn get_substring(s: &[u8], left: usize, right: usize, k: i32, cnts: &Vec<Vec<i32>>) -> i32 {
		let mut set = HashSet::new();
		for i in 0..26 {
			let g = cnts[i][right] - cnts[i][left];
			if g != 0 && g < k {
				set.insert(i as u8);
			}
		}
		if set.is_empty() {
			return (right - left) as i32;
		}
		let mut ret = 0;
		let mut i = left;
		while i < right {
			let mut j = i;
			while j < right && !set.contains(&(s[j] - b'a')) {
				j += 1;
			}
			if i != j {
				ret = ret.max(Solution::get_substring(s, i, j, k, cnts));
				i = j;
			} else {
				i += 1;
			}
		}
		ret
	}
}