struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


impl Solution {
	pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
		let mut r = nums.len();
		let mut ans = i32::MAX;
		while l < r {
			let m = l + (r - l) / 2;
			ans = ans.min(nums[m]);
            if nums[m] < nums[l] {
                r = m;
			} else {
                if nums[r-1] < nums[l] {
					l = m + 1;
				} else {
					r = m;
				}
			}
		}
		ans
	}
}