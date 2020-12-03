struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


impl Solution {
	pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
		let mut ans = vec![0; a.len()];
		let (mut i, mut j) = (0, 1);
		for v in a {
			if v % 2 == 0 {
				ans[i] = v;
				i += 2;
			} else {
				ans[j] = v;
				j += 2;
			}
		}
		ans
	}
}