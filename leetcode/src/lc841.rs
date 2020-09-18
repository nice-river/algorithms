use std::collections::VecDeque;

struct Solution {}

impl Solution {
	pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
		let n = rooms.len();
		let mut vis = vec![false; n];
		let mut que: VecDeque<i32> = VecDeque::new();
		vis[0] = true;
		que.push_back(0);
		while que.len() != 0 {
			let cur = que.pop_front().unwrap();
			for nxt in &rooms[cur as usize] {
				if !vis[*nxt as usize] {
					vis[*nxt as usize] = true;
					que.push_back(nxt.clone());
				}
			}
		}
		vis.iter().all(|&item| item)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sol = Solution {};
		let rooms = vec![
			vec![1, 3], vec![3, 0, 1], vec![2], vec![0]
		];
		println!("{}", Solution::can_visit_all_rooms(rooms));
	}
}