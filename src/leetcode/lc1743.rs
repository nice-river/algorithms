struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::{VecDeque, HashMap, HashSet};

impl Solution {
	pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
		let mut gph = HashMap::new();
		for pair in &adjacent_pairs {
			gph.entry(pair[0]).or_insert(Vec::new()).push(pair[1]);
			gph.entry(pair[1]).or_insert(Vec::new()).push(pair[0]);
		}

		let mut in_que = HashSet::new();
		let mut que = VecDeque::new();
		que.push_back(adjacent_pairs[0][0]);
		que.push_back(adjacent_pairs[0][1]);
		in_que.insert(adjacent_pairs[0][0]);
		in_que.insert(adjacent_pairs[0][1]);
		
		while que.len() != adjacent_pairs.len() + 1 {
			if let Some(nxts) = gph.get(que.front().unwrap()) {
				for nxt in nxts {
					if !in_que.contains(nxt) {
						que.push_front(*nxt);
						in_que.insert(*nxt);
						break;
					}
				}
			}
			if let Some(nxts) = gph.get(que.back().unwrap()) {
				for nxt in nxts {
					if !in_que.contains(nxt) {
						que.push_back(*nxt);
						in_que.insert(*nxt);
						break;
					}
				}
			}
		}

		que.into_iter().collect()
	}
}