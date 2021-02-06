use std::cmp::max;

struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "AABA";
		let k = 0;
		assert_eq!(Solution::character_replacement(s.to_string(), k), 2);
		let s = "ABAA";
		let k = 0;
		assert_eq!(Solution::character_replacement(s.to_string(), k), 2);
	}
}

impl Solution {
	pub fn character_replacement(s: String, k: i32) -> i32 {
		let s = s.into_bytes();
		let mut i = 0;
		let mut j = 0;
		let mut cnt = vec![0; 26];
		let mut maxi = 0;
		while j < s.len() {
			let p = (s[j] - b'A') as usize;
			cnt[p] += 1;
			maxi = std::cmp::max(maxi, cnt[p]);
			j += 1;
			if maxi + k < (j - i) as i32 {
				cnt[(s[i] - b'A') as usize] -= 1;
				i += 1;
			}
		}
		(j - i) as i32
	}
}
