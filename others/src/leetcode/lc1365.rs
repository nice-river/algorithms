use std::process::id;

struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![8,1,2,2,3];
		let ans = Solution::smaller_numbers_than_current(nums);
		assert_eq!(ans, vec![4,0,1,1,3]);
	}
}

struct BinaryIndexedTree {
	arr: Vec<i32>,
}

impl BinaryIndexedTree {
	pub fn new(n: usize) -> Self {
		BinaryIndexedTree {
			arr: vec![0; n + 1]
		}
	}

	pub fn add(self: &mut Self, idx: usize, val: i32) {
		assert!(idx > 0);
		let n = self.arr.len();
		let mut idx = idx;
		while idx < n {
			self.arr[idx] += val;
			let k = idx as i32;
			idx += (k & -k) as usize;
		}
	}

	pub fn get(self: &Self, idx: usize) -> i32 {
		let mut idx = idx;
		let mut ret = 0;
		while idx > 0 {
			ret += self.arr[idx];
			let k = idx as i32;
			idx -= (k & -k) as usize;
		}
		ret
	}
}

impl Solution {
	pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
		let mut bit = BinaryIndexedTree::new(100);
		for num in nums.iter() {
			bit.add(num.clone() as usize + 1, 1);
		}
		let mut ans = vec![0; nums.len()];
		for (idx, num) in nums.iter().enumerate() {
			ans[idx] = bit.get(num.clone() as usize);
		}
		ans
	}
}
