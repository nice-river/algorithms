#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let board = vec![
			vec![-1,-1,-1,-1,-1],
			vec![-1,-1,-1,-1,-1],
			vec![-1,17,-1,-1,13],
			vec![-1,-1,-1,-1,-1],
			vec![-1,15,-1,-1,-1]
		];
		assert_eq!(Solution::snakes_and_ladders(board), 3);
	}
}

struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
	    let mut n = board.len();
	    let mut m = board[0].len();
	    let mut arr = vec![0; n * m + 1];
	    for i in (0..n).rev().step_by(2) {
		    for j in 0..m {
			    let k = (n - 1 - i) * m + j + 1;
			    arr[k] = board[i][j];
		    }
		    if i >= 1 {
			    for j in (0..m).rev() {
				    let k = (n - 1 - (i - 1)) * m + (m - j);
				    arr[k] = board[i - 1][j];
			    }
		    }
	    }
	    let mut vis = vec![false; n * m + 1];
	    let mut que = VecDeque::new();
	    que.push_back((1, 0));
	    vis[1] = true;
	    while !que.is_empty() {
		    let (p, s) = que.pop_front().unwrap();
		    if p == n * m {
			    return s;
		    }
		    for i in 1..=6 {
			    let q = p + i;
			    if q > n * m {
				    break;
			    }
			    if arr[q] == -1 && !vis[q] {
				    que.push_back((q, s + 1));
				    vis[q] = true;
			    } else if arr[q] != -1 && !vis[arr[q] as usize] {
				    que.push_back((arr[q] as usize, s + 1));
				    vis[arr[q] as usize] = true;
			    }
		    }
	    }
	    -1
    }
}