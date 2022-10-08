struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let accounts = vec![vec!["John","johnsmith@mail.com","john_newyork@mail.com"],vec!["John","johnsmith@mail.com","john00@mail.com"],vec!["Mary","mary@mail.com"],vec!["John","johnnybravo@mail.com"]];
		let accounts = accounts.into_iter().map(|a| a.into_iter().map(|s| s.to_string()).collect()).collect();
		dbg!(Solution::accounts_merge(accounts));
	}
}

use std::collections::{HashSet, HashMap};

impl Solution {
	pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
		let n = accounts.len();

		let mut ab = HashMap::new();
		let mut ba = HashMap::new();
		for account in &accounts {
			for s in account {
				if !ab.contains_key(s) {
					let k = ab.len();
					ab.insert(s.clone(), k);
					ba.insert(k, s.clone());
				}
			}
		}

		let mut acct = Vec::with_capacity(n);
		for account in &accounts {
			acct.push(account.iter().map(|s| *ab.get(s).unwrap()).collect::<Vec<usize>>());
		}

		let mut map = HashMap::new();

		for i in 0..n {
			for j in 1..acct[i].len() {
				map.entry(acct[i][j]).or_insert(vec![]).push(i);
			}
		}

		let mut ds = DisjointSet::new(n);
		for (_, arr) in map.into_iter() {
			for i in 1..arr.len() {
				ds.union(arr[0] as i32, arr[i] as i32);
			}
		}

		let mut map = HashMap::new();
		for i in 0..n {
			let p = ds.find(i as i32) as usize;
			for j in 1..acct[i].len() {
				map.entry(p).or_insert(HashSet::new()).insert(acct[i][j].clone());
			}
		}
		map.into_iter().map(|(k, v)| {
			let mut r = vec![ba.get(&acct[k][0]).unwrap().clone()];
			let mut g = v.into_iter().map(
				|k| ba.get(&k).unwrap().clone()
			).collect::<Vec<String>>();
			g.sort_unstable();
			r.extend(g);
			r
		}).collect()
	}
}

#[derive(Debug, Clone)]
struct DisjointSet {
	marks: Vec<i32>,
	size: Vec<i32>,
}

impl DisjointSet {
	fn new(n: usize) -> Self {
		Self {
			marks: vec![-1; n],
			size: vec![1; n],
		}
	}

	fn find(&mut self, x: i32) -> i32 {
		if self.marks[x as usize] != -1 {
			self.marks[x as usize] = self.find(self.marks[x as usize]);
			self.marks[x as usize]
		} else {
			x
		}
	}

	fn union(&mut self, u: i32, v: i32) {
		let u = self.find(u);
		let v = self.find(v);
		if u != v {
			self.marks[v as usize] = u;
			self.size[u as usize] += self.size[v as usize];
		}
	}

	fn get_disjoint_num(&mut self) -> usize {
		let mut set = std::collections::HashSet::new();
		for i in 0..self.marks.len() {
			set.insert(self.find(i as i32));
		}
		set.len()
	}

	fn get_size(&mut self, x: i32) -> i32 {
		let x = self.find(x);
		self.size[x as usize]
	}
}
