#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let ans = Solution::count_range_sum(vec![1,4,-2,3,-4,3,0,-4,4], 3, 6);
		assert_eq!(ans, 16);
		let ans = Solution::count_range_sum(vec![2147483647,-2147483648,-1,0], -1, 0);
		assert_eq!(ans, 4);
		let ans = Solution::count_range_sum(vec![0], 0, 0);
		assert_eq!(ans, 1);
		let ans = Solution::count_range_sum(vec![-2, 5, -1], -2, 2);
		assert_eq!(ans, 3);
		let ans = Solution::count_range_sum(vec![0, 0, 0], 0, 0);
		assert_eq!(ans, 6);
	}
}

struct Solution {}
use std::collections::HashMap;

impl Solution {
	pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
		let lower = lower as i64;
		let upper = upper as i64;
		let n = nums.len();
		if n == 0 {
			return 0;
		}
		let mut s: Vec<(i64, usize)> = vec![(0, 0); n];
		s[0] = (nums[0] as i64, 0);
		for i in 1..n {
			s[i] = (s[i-1].0 + nums[i] as i64, i);
		}
		let mut elem = s.last().unwrap().0;
		s.sort();

		let mut h = HashMap::new();
		for (idx, (v, p)) in s.iter().enumerate() {
			(*h.entry(*v).or_insert(vec![])).push(idx);
		}
		for (_, arr) in h.iter_mut() {
			arr.sort();
		}

		let mut bit = BinaryIndexedTree::new(n + 1);
		let mut ans = 0;
		let mut idx = n as i32 - 1;
		while idx >= 0 {
			let p = h.get_mut(&elem).unwrap().pop().unwrap();
			bit.add(p + 1, 1);
			let a = Solution::lower_bound(&s, elem - upper);
			let b = Solution::lower_bound(&s, elem - lower + 1);
			if lower <= elem && elem <= upper {
				ans += 1;
			}
			if a < s.len() && b > 0 {
				ans += b as i32 - a as i32 - bit.get(b) + bit.get(a);
			}
			elem -= nums[idx as usize] as i64;
			idx -= 1;
		}
		ans
	}

	fn lower_bound(arr: &Vec<(i64, usize)>, val: i64) -> usize {
		let val = (val, 0);
		let mut l = 0;
		let mut r = arr.len();
		let mut ret = r;
		while l < r {
			let m = (l + r) / 2;
			if arr[m] < val {
				l = m + 1;
			} else {
				ret = m;
				r = m;
			}
		}
		if l < arr.len() && val == arr[l] {
			l
		} else {
			ret
		}
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
