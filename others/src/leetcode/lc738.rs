use std::process::id;

struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn monotone_increasing_digits(mut n: i32) -> i32 {
		let mut digits = vec![];
		while n != 0 {
			digits.push(n % 10);
			n /= 10;
		}
		digits.reverse();
		let mut idx = 1;
		while idx < digits.len() && digits[idx] >= digits[idx - 1] {
			idx += 1;
		}
		if idx == digits.len() {
			return digits.iter().fold(0, |num, &d| num * 10 + d);
		}
		idx -= 1;
		while idx >= 1 && digits[idx] == digits[idx - 1] {
			idx -= 1;
		}
		digits[idx] -= 1;
		let mut ans = 0;
		for i in 0..digits.len() {
			ans = ans * 10 + if i > idx { 9 } else { digits[i] };
		}
		ans
	}
}