struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
		let mut s = 0;
		let mut ans = vec![false; a.len()];
		for (i, v) in a.into_iter().enumerate() {
			s = (s * 2 + v) % 5;
			if s == 0 {
				ans[i] = true;
			}
		}
		ans
	}
}
