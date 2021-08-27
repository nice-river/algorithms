struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn find_the_difference(s: String, t: String) -> char {
		let mut ans = 0;
		for &b in s.as_bytes() {
			ans ^= b;
		}
		for &b in t.as_bytes() {
			ans ^= b;
		}
		ans as char
	}
}