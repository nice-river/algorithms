struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let mut nums = vec![0, 1, 0, 1, 0, 1, 99];
		assert_eq!(Solution::single_number(nums), 99);
	}
}


impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut a = 0;
		let mut b = 0;
		for num in nums {
			let tmp = (a & !num) | (b & num);
			b = b ^ num & !a;
			a = tmp;
		}
        b
    }
}