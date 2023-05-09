#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}


impl Solution {
	pub fn is_monotonic(a: Vec<i32>) -> bool {
		let mut ans = true;
		for i in 1..a.len() {
			if a[i] < a[i-1] {
				ans = false;
				break;
			}
		}
		if ans {
			return true;
		}
		ans = true;
		for i in 1..a.len() {
			if a[i] > a[i-1] {
				ans = false;
				break;
			}
		}
		ans
	}
}