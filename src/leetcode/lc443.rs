struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let chars = vec!["a","a","b","b","c","c","c"];
		let mut chars = chars.into_iter().map(|s| s.chars().next().unwrap()).collect();
		let ans = 6;
		assert_eq!(Solution::compress(&mut chars), ans);
	}
}

impl Solution {
	pub fn compress(chars: &mut Vec<char>) -> i32 {
		let mut ans = 0;
		let n = chars.len();
		let mut i = 0;
		while i < n {
			let mut j = i;
			while j < n && chars[j] == chars[i] {
				j += 1;
			}
			chars[ans] = chars[i];
			ans += 1;
			if j - i > 1 {
				for ch in (j - i).to_string().chars() {
					chars[ans] = ch;
					ans += 1;
				}
			}
			i = j;
		}
		ans as i32
	}
}