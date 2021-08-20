struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "leetcode".to_string();
		let ans = "leotcede".to_string();
		assert_eq!(Solution::reverse_vowels(s), ans);
	}
}

impl Solution {
	pub fn reverse_vowels(s: String) -> String {
		let s = s.as_bytes();
		let mut ans = s.to_vec();
		let vowels = vec![b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'];
		let mut vowels_idxes = vec![];
		for (i, ch) in s.iter().enumerate() {
			if vowels.contains(ch) {
				vowels_idxes.push(i);
			}
		}
		for i in 0..vowels_idxes.len() {
			ans[vowels_idxes[i]] = s[vowels_idxes[vowels_idxes.len() - 1 - i]];
		}
		String::from_utf8(ans).unwrap()
	}
}