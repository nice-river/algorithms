struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn check_record(s: String) -> bool {
		let mut absent = 0;
		let mut late = 0;
		for (i, &ch) in s.as_bytes().iter().enumerate() {
			match ch {
				b'A' => absent += 1,
				b'L' => late += 1,
				_ => {}
			}
			if ch != b'L' {
				late = 0;
			}
			if absent >= 2 || late >= 3 {
				return false;
			}
		}
		true
	}
}
