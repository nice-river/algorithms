struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sol = Solution {};
	}
}


impl Solution {
	pub fn largest_perimeter(a: Vec<i32>) -> i32 {
		let mut a = a.clone();
		a.sort();
		for i in (2..a.len()).rev() {
			if a[i] < a[i - 1] + a[i - 2] {
				return a[i] + a[i - 1] + a[i - 2];
			}
		}
		0
	}
}