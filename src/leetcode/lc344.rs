struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


impl Solution {
	pub fn reverse_string(s: &mut Vec<char>) {
		let n = s.len();
		for i in 0..n/2 {
			unsafe {
				std::ptr::swap(&mut s[i], &mut s[n - 1 - i]);
			}
		}
	}
}