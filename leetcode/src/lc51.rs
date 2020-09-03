struct Solution {}

impl Solution {
	pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
		let n = n as usize;
		if n == 1 {
			return vec![vec!["Q".into(); 1]; 1];
		}
		if n <= 3 {
			return vec![];
		}
		let mut ans = Vec::new();
		let mut cur = vec![".".repeat(n); n];
		Solution::dfs(0, &mut cur, &mut ans);
		ans
	}

	fn check(n: usize, m: usize, board: &Vec<String>) -> bool {
		let s = board[0].len();
		for i in 0..n {
			if board[i].as_bytes()[m] == 'Q' as u8 {
				return false;
			}
			if (n - i) + m < s && board[i].as_bytes()[(n - i) + m] == 'Q' as u8 {
				return false;
			}
			if m >= (n - i)  && board[i].as_bytes()[m - (n - i)] == 'Q' as u8 {
				return false;
			}
		}
		true
	}

	fn dfs(n: usize, board: &mut Vec<String>, ans: &mut Vec<Vec<String>>) {
		let s = board[0].len();
		if n == s {
			ans.push(board.clone());
			return
		}
		for i in 0..s {
			if !Solution::check(n, i, board) {
				continue
			}
			board[n].replace_range(i..i+1, "Q");
			Solution::dfs(n + 1, board, ans);
			board[n].replace_range(i..i+1, ".");
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let result = Solution::solve_n_queens(4);
		println!("{}", result.len());
	}
}