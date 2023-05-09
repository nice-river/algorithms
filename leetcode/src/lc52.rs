#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = 4;
		assert_eq!(Solution::total_n_queens(n), 2);
	}
}

struct Solution {}

impl Solution {
	pub fn total_n_queens(n: i32) -> i32 {
		let n = n as usize;
		let mut vis = vec![vec![false; n]; n];
		let mut ans = 0;
		Solution::dfs(0, &mut vis, &mut ans);
		ans
	}

	fn dfs(row: usize, vis: &mut Vec<Vec<bool>>, ans: &mut i32) {
		let n = vis.len();
		if row == n {
			*ans += 1;
			return;
		}
		for col in 0..n {
			if Solution::check(row, col, vis) {
				vis[row][col] = true;
				Solution::dfs(row + 1, vis,  ans);
				vis[row][col] = false;
			}
		}
	}

	fn check(i: usize, j: usize, vis: &Vec<Vec<bool>>) -> bool {
		let n = vis.len() as i32;
		for row in 0..i {
			if vis[row][j] {
				return false;
			}
		}
		let i = i as i32;
		let j = j as i32;
		for y in vec![-1, 1] {
			let x = -1;
			let mut ii = i;
			let mut jj = j;
			loop {
				ii += x;
				jj += y;
				if ii < 0 || ii >= n || jj < 0 || jj >= n {
					break;
				}
				if vis[ii as usize][jj as usize] {
					return false;
				}
			}
		}
		true
	}
}