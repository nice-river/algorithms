struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sol = Solution {};
	}
}


impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
			return 0;
		}
        let mut i = 1;
		let mut j = 1;
		while i < nums.len() {
            if nums[i] > nums[i - 1] {
				i += 1;
			} else {
				while j < nums.len() && nums[j] <= nums[i - 1] {
					j += 1;
				}
				if j == nums.len() {
					break;
				} else {
                    nums.swap(i, j);
					i += 1;
				}
			}
		}
		i as i32
    }
}