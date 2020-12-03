use std::collections::HashSet;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
	pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
		let mut dots = vec![];
		for i in 0..9 {
			for j in 0..9 {
				if board[i][j] == '.' {
					dots.push((i, j));
				}
			}
		}
		
	}
	
	fn get_candidates(i: usize, j: usize, board: &Vec<Vec<char>>) -> HashSet<char> {
		let rng = ('0'..='9').collect::<Vec<char>>().into_iter();
	}
	
	fn dfs(idx: usize, dots: &Vec<(usize, usize)>, board: &mut Vec<Vec<char>>) -> bool {
		if idx == dots.len() {
			return true;
		}
		
		let (i, j) = dots[idx];
		
		false
	}
}