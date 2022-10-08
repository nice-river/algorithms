struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn remove_duplicates(s: String) -> String {
		let s = s.as_bytes();
		let mut stk = Vec::new();
		for &ch in s.iter() {
			if !stk.is_empty() && stk.last().unwrap() == &ch {
				stk.pop();
			} else {
				stk.push(ch);
			}
		}
		String::from_utf8(stk).unwrap()
	}
}