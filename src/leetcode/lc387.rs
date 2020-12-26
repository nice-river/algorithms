struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::first_uniq_char("ab".to_string()), 0);
	}
}

impl Solution {
	pub fn first_uniq_char(s: String) -> i32 {
		let mut map = vec![-1; 128];
		for (i, &b) in s.as_bytes().iter().enumerate() {
			let k = b as usize;
			if map[k] == -1 {
				map[k] = i as i32;
			} else {
				map[k] = s.len() as i32;
			}
		}
		let mut ans = s.len() as i32;
		for k in b'a'..=b'z' {
			let k = k as usize;
			if map[k] != -1 {
				ans = std::cmp::min(ans, map[k]);
			}
		}
		if ans == s.len() as i32 { -1 } else { ans }
	}
}
