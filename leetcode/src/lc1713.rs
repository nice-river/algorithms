struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let target = vec![5, 1, 3];
		let arr = vec![9, 4, 2, 3, 4];
		let ans = 2;
		assert_eq!(Solution::min_operations(target, arr), ans);
	}

	#[test]
	fn test1() {
		let target = vec![6, 4, 8, 1, 3, 2];
		let arr = vec![4, 7, 6, 2, 3, 8, 6, 1];
		let ans = 3;
		assert_eq!(Solution::min_operations(target, arr), ans);
	}

	#[test]
	fn test2() {
		let target = vec![16,7,20,11,15,13,10,14,6,8];
		let arr = vec![11,14,15,7,5,5,6,10,11,6];
		let ans = 6;
		assert_eq!(Solution::min_operations(target, arr), ans);
	}

	#[test]
	fn test3() {
		let target = vec![17,18,14,13,6,9,1,3,2,20];
		let arr = vec![18,15,14,6,6,13,15,20,2,6];
		let ans = 6;
		assert_eq!(Solution::min_operations(target, arr), ans);
	}

	#[test]
	fn test4() {
		let target = vec![19,15,2,3,10,6,7,4,8,14];
		let arr = vec![9,7,9,2,15,14,3,8,14,8];
		let ans = 6;
		assert_eq!(Solution::min_operations(target, arr), ans);
	}
}


use std::collections::HashMap;


impl Solution {
	pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
		let mut map = HashMap::new();
		for (idx, t) in target.into_iter().enumerate() {
			map.insert(t, idx);
		}
		let mut pos = Vec::with_capacity(arr.len());
		for num in arr {
			if let Some(x) = map.get(&num) {
				pos.push(*x);
			}
		}
		if pos.len() == 0 {
			return map.len() as i32;
		}
		let mut arr = Vec::with_capacity(pos.len());
		arr.push(pos[0]);
		for num in pos.into_iter().skip(1) {
			if &num > arr.last().unwrap() {
				arr.push(num);
			} else {
				let mut l = 0;
				let mut r = arr.len() - 1;
				while l < r {
					let m = (l + r) / 2;
					if arr[m] < num {
						l = m + 1;
					} else {
						r = m;
					}
				}
				arr[l] = num;
			}
		}
		(map.len() - arr.len()) as i32
	}
}