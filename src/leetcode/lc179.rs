struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![432, 43243];
		assert_eq!(Solution::largest_number(nums), String::from("43243432"))
	}
}


use std::cmp::Ordering;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
		let mut nums: Vec<_> = nums.into_iter().map(|x| x.to_string()).collect();
        nums.sort_unstable_by(|a, b| {
            let x = format!("{}{}", a, b);
			let y = format!("{}{}", b, a);
            y.cmp(&x)
		});
		let nums: String = nums.join("");
        for (i, &x) in nums.as_bytes().iter().enumerate() {
            if x != b'0' {
				return nums.split_at(i).1.to_string();
			}
		}
        "0".to_string()
	}
}