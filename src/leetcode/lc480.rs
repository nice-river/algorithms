struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = [1,3,-1,-3,5,3,6,7];
		let k = 3;
		let ans = [1,-1,-1,3,5,6];
		let ans: Vec<f64> = ans.to_vec().into_iter().map(|v| v as f64).collect();
		assert_eq!(Solution::median_sliding_window(nums.to_vec(), k), ans);
	}
}

use std::collections::BTreeSet;

impl Solution {
	pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
		let k = k as usize;
		let mut middle = Middle::new(k);
		let mut ans = Vec::with_capacity(nums.len() - k + 1);
		for (i, &num) in nums.iter().enumerate() {
			middle.insert((num, i));
			if i >= k - 1 {
				ans.push(middle.get_middle());
				middle.remove(&(nums[i + 1 - k], i + 1 - k));
			}
		}
		ans
	}
}


#[derive(Debug, Clone)]
struct Middle {
	length: usize,
	left: BTreeSet<(i32, usize)>,
	right: BTreeSet<(i32, usize)>,
}

impl Middle {
	pub fn new(k: usize) -> Self {
		Self {
			length: k,
			left: BTreeSet::new(),
			right: BTreeSet::new(),
		}
	}

	pub fn insert(&mut self, val: (i32, usize)) {
		self.left.insert(val);
		if self.left.len() > self.length / 2 {
			let last = self.left.iter().next_back().unwrap().clone();
			self.left.remove(&last);
			self.right.insert(last);
		}

		if !self.left.is_empty() && !self.right.is_empty() {
			let a = self.left.iter().next_back().unwrap().clone();
			let b = self.right.iter().next().unwrap().clone();
			if a > b {
				self.left.remove(&a);
				self.right.remove(&b);
				self.left.insert(b);
				self.right.insert(a);
			}
		}
	}

	pub fn remove(&mut self, val: &(i32, usize)) {
		if self.left.is_empty() {
			self.right.remove(val);
		} else {
			let a = self.left.iter().next_back().unwrap();
			if val > a {
				self.right.remove(val);
			} else {
				self.left.remove(val);
			}
		}
	}

	pub fn get_middle(&self) -> f64 {
		let b = self.right.iter().next().unwrap();
		return if self.length % 2 == 1 {
			b.0 as f64
		} else {
			let a = self.left.iter().next_back().unwrap();
			(a.0 as f64 + b.0 as f64) / 2.0
		}
	}
}