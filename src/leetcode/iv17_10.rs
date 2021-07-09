struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![6, 6, 6, 7, 7, 7, 7];
        assert_eq!(Solution::majority_element(nums), 7);
	}
}


impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
		let mut cnt = 0;
		let mut x = -1;
        for &num in nums.iter() {
			if cnt == 0 {
				x = num;
				cnt = 1;
			} else {
				cnt += if num == x { 1 } else { -1 };
			}
		}
		if nums.iter().filter(|&&e| e == x).count() * 2 > nums.len() {
			x
		} else {
			-1
		}
    }
}