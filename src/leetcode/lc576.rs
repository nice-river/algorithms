#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let m = 2;
		let n = 2;
		let max_move = 2;
		let start_row = 0;
		let start_column = 0;
		let ans = 6;
		assert_eq!(Solution::find_paths(m, n, max_move, start_row, start_column), ans);
	}

	#[test]
	fn test1() {
		let m = 1;
		let n = 3;
		let max_move = 3;
		let start_row = 0;
		let start_column = 1;
		let ans = 12;
		assert_eq!(Solution::find_paths(m, n, max_move, start_row, start_column), ans);
	}

	#[test]
	fn test2() {
		let m = 8;
		let n = 50;
		let max_move = 23;
		let start_row = 5;
		let start_column = 26;
		let ans = 914783380;
		assert_eq!(Solution::find_paths(m, n, max_move, start_row, start_column), ans);
	}
}

struct Solution {}

impl Solution {
	pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
		let m = m as usize;
		let n = n as usize;
		let mut dp = vec![vec![vec![0; n]; m]; 2];
		let mut in_stk = vec![vec![false; n]; m];
		let mut stk = vec![vec![]; 2];
		let mut cur = 0;
		dp[cur][start_row as usize][start_column as usize] = 1;
		in_stk[start_row as usize][start_column as usize] = true;
		stk[cur].push((start_row, start_column));
		let modulo = 1000000007;
		let mut ans = 0;
		let dirs = [-1, 0, 1, 0, -1];
		for _ in 0..max_move {
			let nxt = cur ^ 1;
			dp[nxt].iter_mut().for_each(|row| row.iter_mut().for_each(|x| *x = 0));
			while let Some((x, y)) = stk[cur].pop() {
				let cnt = dp[cur][x as usize][y as usize];
				in_stk[x as usize][y as usize] = false;
				for d in 0..4 {
					let p = x + dirs[d];
					let q = y + dirs[d + 1];
					if p < 0 || q < 0 || p == m as i32 || q == n as i32 {
						ans += cnt;
						ans %= modulo;
					} else {
						dp[nxt][p as usize][q as usize] += cnt;
						dp[nxt][p as usize][q as usize] %= modulo;
						if !in_stk[p as usize][q as usize] {
							stk[nxt].push((p, q));
							in_stk[p as usize][q as usize] = true;
						}
					}
				}
			}
			cur ^= 1;
		}
		ans as i32
	}
}