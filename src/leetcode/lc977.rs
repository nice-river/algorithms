#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}



impl Solution {
	pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
		let mut ans = Vec::with_capacity(a.len());
		let mut i = 0;
		let mut j = a.len() - 1;
		while i < j + 1{
			if a[i].abs() <= a[j].abs() {
				ans.push(a[j] * a[j]);
				j -= 1;
			} else {
				ans.push(a[i] * a[i]);
				i += 1;
			}
		}
		ans.reverse();
		ans
	}
}