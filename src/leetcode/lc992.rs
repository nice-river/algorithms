struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let arr = [1,2,1,2,3];
		let k = 2;
		assert_eq!(Solution::subarrays_with_k_distinct(arr.to_vec(), k), 7);

		let arr = [1,2,1,3,4];
		let k = 3;
		assert_eq!(Solution::subarrays_with_k_distinct(arr.to_vec(), k), 3);
	}
}

use std::collections::HashMap;

impl Solution {
	pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
		let k = k as usize;
		let mut longest = HashMap::new();
		let mut shortest = HashMap::new();
		let mut ans = 0;
		let (mut i, mut j) = (0, 0);
		let mut p = 0;
		while p < a.len() && longest.len() != k {
			*longest.entry(a[p]).or_insert(0) += 1;
			*shortest.entry(a[p]).or_insert(0) += 1;
			p += 1;
		}
		if longest.len() < k {
			return 0;
		}
		while shortest.get(&a[j]).unwrap() > &1 {
			*shortest.get_mut(&a[j]).unwrap() -= 1;
			j += 1;
		}
		ans += j - i + 1;

		while p < a.len() {
			*longest.entry(a[p]).or_insert(0) += 1;
			*shortest.entry(a[p]).or_insert(0) += 1;
			while i < a.len() && longest.len() > k {
				*longest.get_mut(&a[i]).unwrap() -= 1;
				if longest.get(&a[i]).unwrap() == &0 {
					longest.remove(&a[i]);
				}
				i += 1;
			}
			while j < a.len() && shortest.len() > k {
				*shortest.get_mut(&a[j]).unwrap() -= 1;
				if shortest.get(&a[j]).unwrap() == &0 {
					shortest.remove(&a[j]);
				}
				j += 1;
			}
			while j < a.len() && shortest.get(&a[j]).unwrap() > &1 {
				*shortest.get_mut(&a[j]).unwrap() -= 1;
				j += 1;
			}
			if longest.len() == k {
				ans += j - i + 1;
			}
			p += 1;
		}

		ans as i32
	}
}