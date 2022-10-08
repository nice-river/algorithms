struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		let s = s.as_bytes();
		let mut st = 0;
		while st < s.len() {
			let mut ed = st;
			while ed + 1 < s.len() && s[ed + 1] == s[st] {
				ed += 1;
			}
			if ed - st >= 2 {
				ans.push(vec![st as i32, ed as i32]);
			}
			st = ed + 1;
		}
		ans
	}
}