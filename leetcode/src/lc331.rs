struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "9,3,4,#,#,1,#,#,2,#,6,#,#";
		assert_eq!(Solution::is_valid_serialization(s.to_string()), true);
	}

	#[test]
	fn test2() {
		let s = "1,#";
		assert_eq!(Solution::is_valid_serialization(s.to_string()), false);
	}

	#[test]
	fn test3() {
		let s = "9,#,#,1";
		assert_eq!(Solution::is_valid_serialization(s.to_string()), false);
	}

	#[test]
	fn test4() {
		let s = "9";
		assert_eq!(Solution::is_valid_serialization(s.to_string()), false);
		let s = "#";
		assert_eq!(Solution::is_valid_serialization(s.to_string()), true);
		let s = "#,1";
		assert_eq!(Solution::is_valid_serialization(s.to_string()), false);
		let s = "1,,,1";
		assert_eq!(Solution::is_valid_serialization(s.to_string()), false);
		let s = "1,#,#,";
		assert_eq!(Solution::is_valid_serialization(s.to_string()), false);
		let s = "1#,#";
		assert_eq!(Solution::is_valid_serialization(s.to_string()), false);
	}
}

impl Solution {
	pub fn is_valid_serialization(preorder: String) -> bool {
		let preorder = preorder.as_bytes();
		let ret = Solution::dfs(preorder, 0);
		ret.0 && ret.1 == preorder.len()
	}

	fn dfs(s: &[u8], mut idx: usize) -> (bool, usize) {
		if idx == s.len() {
			return (false, 0);
		}
		match s[idx] {
			b'#' => {
				(true, idx + 1)
			}
			b'0'..=b'9' => {
				while idx < s.len() && b'0' <= s[idx] && s[idx] <= b'9' {
					idx += 1;
				}
				if idx == s.len() || s[idx] != b',' {
					return (false, 0);
				}
				let left = Solution::dfs(s, idx + 1);
				if !left.0 {
					return left;
				}
				if left.1 == s.len() || s[left.1] != b',' {
					return (false, 0);
				}
				Solution::dfs(s, left.1 + 1)
			}
			_ => {
				(false, 0)
			}
		}
	}
}