struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let deadends = vec![String::from("8888")];
		let target = String::from("0009");
		assert_eq!(Solution::open_lock(deadends, target), 1);
	}
}

use std::collections::VecDeque;
use std::collections::HashSet;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
		let deadends = deadends.into_iter().map(|s| {
			s.into_bytes().into_iter().map(|b| (b - b'0') as i8).collect::<Vec<_>>()
		}).collect::<Vec<_>>();
		let mut dead_ends = HashSet::new();
        for deadend in deadends {
            dead_ends.insert(deadend);
		}
		let target = target.into_bytes().into_iter().map(|b| (b - b'0') as i8).collect::<Vec<_>>();
		let start = vec![0; 4];
        if dead_ends.contains(&target) || dead_ends.contains(&start) {
			return -1;
		}
		let mut vis = HashSet::new();
		let mut que = VecDeque::new();
		que.push_back((start.clone(), 0));
		vis.insert(start);
		while !que.is_empty() {
			let (cur, step) = que.pop_front().unwrap();
			if cur == target {
				return step;
			}
			let mut nxt = cur.clone();
            for i in 0..4 {
				nxt[i] = (nxt[i] + 1 + 10) % 10;
				if !vis.contains(&nxt) && !dead_ends.contains(&nxt) {
					que.push_back((nxt.clone(), step + 1));
					vis.insert(nxt.clone());
				}
				nxt[i] = (nxt[i] - 2 + 10) % 10;
				if !vis.contains(&nxt) && !dead_ends.contains(&nxt) {
					que.push_back((nxt.clone(), step + 1));
					vis.insert(nxt.clone());
				}
				nxt[i] = (nxt[i] + 1 + 10) % 10;
			}
		}
		-1
    }
}