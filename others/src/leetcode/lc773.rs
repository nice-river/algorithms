#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Node {
	board: Vec<Vec<i32>>,
	zero: (usize, usize),
}

impl Node {
	fn new(board: Vec<Vec<i32>>) -> Self {
		let mut x = 0;
		let mut y = 0;
		for i in 0..2 {
			for j in 0..3 {
				if board[i][j] == 0 {
					x = i;
					y = j;
					break;
				}
			}
		}
		Self {
			board,
			zero: (x, y),
		}
	}

	fn go(&self, dir_x: i32, dir_y: i32) -> Option<Self> {
		let x = self.zero.0 as i32 + dir_x;
		let y = self.zero.1 as i32 + dir_y;
		if x < 0 || y < 0 || x >= 2 || y >= 3 {
			return None;
		}
		let x = x as usize;
		let y = y as usize;
		let mut board = self.board.clone();
		let t = board[x][y];
		board[x][y] = 0;
		board[self.zero.0][self.zero.1] = t;
		Some(Self {
			board,
			zero: (x, y),
		})
	}
}

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
	    let mut map = vec![HashMap::new(), HashMap::new()];
	    let start = Node::new(board);
	    map[0].insert(start.clone(), 0);
	    let target = Node::new(vec![vec![1, 2, 3], vec![4, 5, 0]]);
	    map[1].insert(target.clone(), 0);
	    let mut que = vec![VecDeque::new(), VecDeque::new()];
	    que[0].push_back((start, 0));
	    que[1].push_back((target, 0));
	    let dirs = vec![-1, 0, 1, 0, -1];
	    while !que[0].is_empty() && ! que[1].is_empty() {
		    let idx = if que[0].len() < que[1].len() {
			    0
		    } else {
			    1
		    };
		    let (node, step) = que[idx].pop_front().unwrap();
		    if let Some(&k) = map[idx ^ 1].get(&node) {
			    return k + step;
		    }
		    for i in 0..4 {
			    let dir_x = dirs[i];
			    let dir_y = dirs[i + 1];
			    if let Some(nxt_node) = node.go(dir_x, dir_y) {
				    if !map[idx].contains_key(&nxt_node) {
					    que[idx].push_back((nxt_node.clone(), step + 1));
					    map[idx].insert(nxt_node, step + 1);
				    }
			    }
		    }
	    }
	    -1
    }
}