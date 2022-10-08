struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let words = vec!["aaaa","asas","able","ability","actt","actor","access"];
		let puzzles = vec!["aboveyz","abrodyz","abslute","absoryz","actresz","gaswxyz"];
		let words = words.into_iter().map(|s| s.to_string()).collect();
		let puzzles = puzzles.into_iter().map(|s| s.to_string()).collect();
		let ans = vec![1,1,3,2,4,0];
		assert_eq!(Solution::find_num_of_valid_words(words, puzzles), ans);
	}

	#[test]
	fn test1() {
		let words = vec!["ab", "ba", "abc", "cba"];
		let puzzles = vec!["abc", "ab", "cba"];
		let words = words.into_iter().map(|s| s.to_string()).collect();
		let puzzles = puzzles.into_iter().map(|s| s.to_string()).collect();
		let ans = vec![4, 2, 2];
		assert_eq!(Solution::find_num_of_valid_words(words, puzzles), ans);
	}
}

use std::collections::HashMap;

impl Solution {
	pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
		let words = words.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
		let puzzles = puzzles.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
		let mut map = HashMap::new();

		for &word in words.iter() {
			let s = Solution::map_to_int(word);
			*map.entry(s).or_insert(0) += 1;
		}

		let mut ans = Vec::with_capacity(puzzles.len());
		for &puzzle in puzzles.iter() {
			let mut s = Solution::map_to_int(&puzzle[1..]);
			let mut t = s;
			let mut k = *map.get(&(1 << (puzzle[0] - b'a'))).unwrap_or(&0);
			while t != 0 {
				k += *map.get(&(t | (1 << (puzzle[0] - b'a')))).unwrap_or(&0);
				t = (t - 1) & s;
			}
			ans.push(k);
		}
		ans
	}

	fn map_to_int(s: &[u8]) -> u32 {
		let mut ret = 0;
		for (i, &b) in s.iter().enumerate() {
			ret |= 1 << (b - b'a');
		}
		ret
	}
}