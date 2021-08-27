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
        let mut ans = i32::MAX;
        let (mut l, mut r) = (0, nums.len());
		while l < r {
            let m = l + (r - l) / 2;
			ans = ans.min(nums[m]);
            if nums[l] == nums[m] && nums[m] == nums[r-1] {
                l += 1;
				r -= 1;
			} else if nums[l] < nums[r-1] {
				r = m;
			} else {
				if nums[m] < nums[l] {
                    r = m;
				} else {
					l = m + 1;
				}
			}
		}
		ans
    }
}