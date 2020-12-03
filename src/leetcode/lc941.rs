struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn valid_mountain_array(a: Vec<i32>) -> bool {
		if a.len() < 3 {
			false
		} else {
			let mut i = 0;
			while i + 1 < a.len() && a[i] < a[i + 1] {
				i += 1;
			}
			if i == 0 || i + 1 == a.len() || a[i] == a[i - 1] {
				false
			} else {
				while i + 1 < a.len() && a[i] > a[i + 1] {
					i += 1;
				}
				i + 1 == a.len()
			}
		}
	}
}