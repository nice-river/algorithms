struct Solution {}

impl Solution {
	pub fn reverse_words(s: String) -> String {
		let mut iter = s.split_ascii_whitespace();
		let mut vec = Vec::new();
		while let Some(s) = iter.next() {
			vec.push(s.chars().rev().collect::<String>());
		}
		vec.join(" ")
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "Let's take LeetCode contest".into();
		let t: String = "s'teL ekat edoCteeL tsetnoc".into();
		assert_eq!(t, Solution::reverse_words(s));
	}
}
