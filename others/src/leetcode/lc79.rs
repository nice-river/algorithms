struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let board = vec![vec!['A', 'B', 'C', 'E'], vec!['S', 'F', 'E', 'S'], vec!['A', 'D', 'E', 'E']];
		assert!(Solution::exist(board, "ABCESEEEFS".into()));
	}
}


impl Solution {
	pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
		let n = board.len();
		if n == 0 {
			return false;
		}
		let m = board[0].len();
		let mut vis = vec![vec![false; m]; n];
		let word = word.chars().collect::<Vec<char>>();
		let m = board[0].len();
		for (i, row) in board.iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {
				if cell == &word[0] {
					vis[i][j] = true;
					if Solution::dfs(1, i as i32, j as i32, n, m, &board, &word, &mut vis) {
						return true;
					}
					vis[i][j] = false;
				}
			}
		}
		return false;
	}

	fn dfs(idx: i32, x: i32, y: i32, n: usize, m: usize, board: &Vec<Vec<char>>, word: &Vec<char>, vis: &mut Vec<Vec<bool>>) -> bool {
		if idx == word.len() as i32 {
			return true;
		}
		let dirs: Vec<i32> = vec![0, -1, 0, 1, 0];
		for i in 0..4 {
			let nx = x as i32 + dirs[i];
			let ny = y as i32 + dirs[i + 1];
			if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 || vis[nx as usize][ny as usize] || board[nx as usize][ny as usize] != word[idx as usize] {
				continue ;
			}
			vis[nx as usize][ny as usize] = true;
			if Solution::dfs(idx + 1, nx, ny, n, m, board, word, vis) {
				return true;
			}
			vis[nx as usize][ny as usize] = false;
		}
		return false;
	}
}