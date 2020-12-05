struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums1 = vec![3, 4, 6, 5];
		let nums2 = vec![9, 1, 2, 5, 8, 3];
		let k = 5;
		assert_eq!(Solution::max_number(nums1, nums2, k), vec![9, 8, 6, 5, 3])
	}

	#[test]
	fn test1() {
		let nums1 = vec![6, 7];
		let nums2 = vec![6, 0, 4];
		let k = 5;
		assert_eq!(Solution::max_number(nums1, nums2, k), vec![6, 7, 6, 0, 4]);
	}

	#[test]
	fn test2() {
		let nums1 = vec![3, 9];
		let nums2 = vec![8, 9];
		let k = 3;
		assert_eq!(Solution::max_number(nums1, nums2, k), vec![9, 8, 9])
	}

	#[test]
	fn test3() {
		let nums1 = vec![2,5,6,4,4,0];
		let nums2 = vec![7,3,8,0,6,5,7,6,2];
		let k = 15;
		assert_eq!(Solution::max_number(nums1, nums2, k), vec![7,3,8,2,5,6,4,4,0,6,5,7,6,2,0]);
	}

	#[test]
	fn test4() {
		let nums1 = vec![1, 2, 3];
		let nums2 = vec![7, 8, 9];
		let k = 3;
		assert_eq!(Solution::max_number(nums1, nums2, k), vec![9, 2, 3]);

		let nums1 = vec![1, 2, 3];
		let nums2 = vec![7, 8, 9];
		let k = 4;
		assert_eq!(Solution::max_number(nums1, nums2, k), vec![9, 1, 2, 3]);

		let nums1 = vec![1, 2, 3];
		let nums2 = vec![7, 8, 9];
		let k = 5;
		assert_eq!(Solution::max_number(nums1, nums2, k), vec![8, 9, 1, 2, 3]);

	}

	#[test]
	fn test5() {
		let nums1 = vec![0, 0, 0];
		let nums2 = vec![0, 0, 0];
		let k = 4;
		assert_eq!(Solution::max_number(nums1, nums2, k), vec![0, 0, 0, 0]);
	}

	#[test]
	fn test6() {
		let nums1 = vec![];
		let nums2 = vec![9, 3, 2, 6, 7, 5];
		let k = 4;
		assert_eq!(Solution::max_number(nums1, nums2, k), vec![9, 6, 7, 5]);

		let nums1 = vec![];
		let nums2 = vec![9, 3, 2, 6, 7, 5];
		let k = 6;
		assert_eq!(Solution::max_number(nums1, nums2, k), vec![9, 3, 2, 6, 7, 5]);

		let nums2 = vec![];
		let nums1 = vec![9, 3, 2, 6, 7, 5];
		let k = 6;
		assert_eq!(Solution::max_number(nums1, nums2, k), vec![9, 3, 2, 6, 7, 5]);
	}

	#[test]
	fn test7() {
		let nums1 = vec![2,1,7,8,0,1,7,3,5,8,9,0,0,7,0,2,2,7,3,5,5];
		let nums2 = vec![2,6,2,0,1,0,5,4,5,5,3,3,3,4];
		let k = 35;
		assert_eq!(Solution::max_number(nums1, nums2, k), vec![2,6,2,2,1,7,8,0,1,7,3,5,8,9,0,1,0,5,4,5,5,3,3,3,4,0,0,7,0,2,2,7,3,5,5]);
	}
}

impl Solution {
	pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
		let k = k as usize;
		let h1 = Solution::build(&nums1, k);
		let h2 = Solution::build(&nums2, k);
		let mut ans = match (h1[k].is_empty(), h2[k].is_empty()) {
			(false, true) => h1[k].clone(),
			(true, false) => h2[k].clone(),
			(false, false) => h1[k].clone().min(h2[k].clone()).clone(),
			(true, true) => vec![0; k],
		};
		for s in 1..k {
			let t = k - s;
			if h1[s].is_empty() || h2[t].is_empty() {
				continue;
			}
			let arr = [&h1[s], &h2[t]];
			let mut idx = [0, 0];
			let mut k = 0;
			let mut m = 0;
			while idx[0] < s && idx[1] < t {
				let mut a = idx[0];
				let mut b = idx[1];
				let mut p = 0;
				while a < s && b < t {
					if arr[0][a] < arr[1][b] {
						p = 1;
						break;
					} else if arr[0][a] > arr[1][b] {
						p = 0;
						break;
					} else {
						a += 1;
						b += 1;
					}
				}
				if a == s {
					p = 1;
				}
				if b == t {
					p = 0;
				}

				if arr[p][idx[p]] < ans[k] && m != 1 {
					m = -1;
					break;
				}
				if arr[p][idx[p]] > ans[k] {
					m = 1;
				}
				ans[k] = arr[p][idx[p]];
				idx[p] += 1;
				k += 1;
			}
			for p in 0..2 {
				while idx[p] < arr[p].len() {
					if arr[p][idx[p]] < ans[k] && m != 1 {
						m = -1;
						break;
					}
					if arr[p][idx[p]] > ans[k] {
						m = 1;
					}
					ans[k] = arr[p][idx[p]];
					idx[p] += 1;
					k += 1;
				}
			}
		}
		ans
	}

	fn build(arr: &Vec<i32>, k: usize) -> Vec<Vec<i32>> {
		let n = arr.len();
		let mut ret = vec![vec![]; k + 1];
		if arr.is_empty() {
			return ret;
		}
		let mut nxt = vec![vec![n; 10]; n];
		nxt.last_mut().unwrap()[*arr.last().unwrap() as usize] = n - 1;
		for i in (0..n-1).rev() {
			nxt[i] = nxt[i + 1].clone();
			nxt[i][arr[i] as usize] = i;
		}
		for i in 1..k+1 {
			if arr.len() < i {
				continue;
			}
			let mut p = 0;
			while ret[i].len() < i {
				for j in (0..10).rev() {
					if n - nxt[p][j] >= i - ret[i].len() {
						ret[i].push(j as i32);
						p = nxt[p][j] + 1;
						break;
					}
				}
			}
		}
		ret
	}
}
