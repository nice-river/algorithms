struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![3, 1];
		assert_eq!(Solution::reverse_pairs(nums), 1);
	}
}

impl Solution {
	pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
		if nums.len() <= 1 {
			return 0;
		}
		let mut nums = nums.clone();
		let mut tmp = vec![0; nums.len()];
		let mut ans = 0;
		let l = nums.len();
		Solution::merge_sort(&mut nums, 0, l, &mut ans, &mut tmp);
		ans
	}

	fn merge_sort(nums: &mut Vec<i32>, s: usize, e: usize, ans: &mut i32, tmp: &mut Vec<i32>) {
		if e - s == 1 {
			return ;
		}
		let m = s + (e - s) / 2;
		Solution::merge_sort(nums, s, m, ans, tmp);
		Solution::merge_sort(nums, m, e, ans, tmp);
		let (mut i, mut j) = (s, m);
		let mut k = s;
		let mut p = m;
		while i < m && j < e {
			while p < e && nums[i] as i64 > 2 * nums[p] as i64 {
				p +=1;
			}
			if nums[i] >= nums[j] {
				tmp[k] = nums[j];
				j += 1;
			} else {
				tmp[k] = nums[i];
				i += 1;
				*ans += (p - m) as i32;
			}
			k += 1;
		}
		while i < m {
			while p < e && nums[i] as i64 > 2 * nums[p] as i64 {
				p +=1;
			}
			tmp[k] = nums[i];
			*ans += (p - m) as i32;
			i += 1;
			k += 1;
		}
		while j < e {
			tmp[k] = nums[j];
			j += 1;
			k += 1;
		}
		for i in s..e {
			nums[i] = tmp[i];
		}
	}
}