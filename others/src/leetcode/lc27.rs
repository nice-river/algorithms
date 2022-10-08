struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
		let mut j = nums.len();
		while i < j {
            if nums[i] != val {
				i += 1;
			} else {
				nums.swap(i, j - 1);
				j -= 1;
			}
		}
		j as i32
    }
}