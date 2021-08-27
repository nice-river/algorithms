struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		// let ring = "godding".to_string();
		// let key = "gd".to_string();
		// let ring = "caotmcaataijjxi".to_string();
		// let key = "oatjiioicitatajtijciocjcaaxaaatmctxamacaamjjx".to_string();
		let ring = "fhonicxatucjkkgufleyczaoa".to_string();
		let key = "azyttgfecgkgacfixhekaackfujnookntyehcnculfkaxecgotahfuckiiycoozfaaloicfhjauojlcukazcnuaoxjlcuzxufyka".to_string();
		assert_eq!(Solution::find_rotate_steps(ring, key), 536);
	}
}

use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Ordering;

impl Solution {
	pub fn find_rotate_steps(ring: String, key: String) -> i32 {
		let ring = ring.into_bytes();
		let key = key.into_bytes();
		let mut pos = vec![vec![]; 256];
		let n = ring.len();
		for (p, &r) in ring.iter().enumerate() {
			pos[r as usize].push(p);
		}
		let mut dist = vec![vec![n * 2; n]; n];
		for i in 0..n {
			for j in i+1..n {
				dist[i][j] = std::cmp::min(j - i, n - j + i);
				dist[j][i] = dist[i][j];
			}
		}
		let mut que = BinaryHeap::new();
		let mut vis = vec![vec![1usize << 31; key.len() + 1]; n];
		que.push(Node {r: 0, k: 0, s: 0});
		vis[0][0] = 0;
		while !que.is_empty() {
			let node = que.pop().unwrap();
			if node.k == key.len() {
				return node.s as i32;
			}
			if ring[node.r] == key[node.k] {
				let nxt = Node::new(node.r, node.k + 1, node.s + 1);
				if nxt.s < vis[nxt.r][nxt.k] {
					vis[nxt.r][nxt.k] = nxt.s;
					que.push(nxt);
				}
			} else {
				for &p in pos[key[node.k] as usize].iter() {
					let nxt = Node::new(p, node.k + 1, node.s + dist[node.r][p] + 1);
					if nxt.s < vis[nxt.r][nxt.k] {
						vis[nxt.r][nxt.k] = nxt.s;
						que.push(nxt);
					}
				}
			}
		}
		0
	}
}

#[derive(Copy, Clone, Debug, Hash)]
struct Node {
	r: usize,
	k: usize,
	s: usize,
}

impl Node {
	fn new(r: usize, k: usize, s: usize) -> Self {
		Self {r, k, s}
	}
}

impl PartialEq for Node {
	fn eq(&self, other: &Self) -> bool {
		self.r == other.r && self.k == other.k && self.s == other.s
	}
}

impl Eq for Node {}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some((self.s).cmp(&other.s).reverse())
	}
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		(self.s).cmp(&other.s).reverse()
	}
}

