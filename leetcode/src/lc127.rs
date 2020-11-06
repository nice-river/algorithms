struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::VecDeque;

impl Solution {
	pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
		let begin_word = begin_word.as_bytes();
		let end_word = end_word.as_bytes();
		let mut word_list: Vec<&[u8]> = word_list.iter().map(|w| w.as_bytes()).collect();
		word_list.push(begin_word);
		let target = word_list.iter().position(|&x| x == end_word);
		if target.is_none() {
			return 0;
		}
		let target = target.unwrap();
		let n = word_list.len();
		let mut vis = vec![false; n];
		let mut que = VecDeque::new();
		que.push_back((n - 1, 1));
		vis[n - 1] = true;

		while !que.is_empty() {
			let (cur_idx, steps) = que.pop_front().unwrap();
			if cur_idx == target {
				return steps;
			}
			for i in 0..n-1 {
				if !vis[i] && word_list[i].len() == word_list[cur_idx].len() {
					let cnt  = word_list[i].iter().zip(word_list[cur_idx]).filter(|(x, y)| x != y).count();
					if cnt == 1 {
						que.push_back((i, steps + 1));
						vis[i] = true;
					}
				}
			}
		}
		0
	}
}
