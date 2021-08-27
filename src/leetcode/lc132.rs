#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "aab";
		assert_eq!(Solution::min_cut(s.to_string()), 1);
	}

	#[test]
	fn test1() {
		let s = "a";
		assert_eq!(Solution::min_cut(s.to_string()), 0);

		let s = "ab";
		assert_eq!(Solution::min_cut(s.to_string()), 1);

		let s = "aa";
		assert_eq!(Solution::min_cut(s.to_string()), 0);
	}
}

struct Solution {}

impl Solution {
	pub fn min_cut(s: String) -> i32 {
		let s = s.as_bytes();
		let mut memo = vec![vec![i32::max_value(); s.len() + 1]; s.len() + 1];
		Solution::dp(0, s.len(), s, &mut memo);
		memo[0][s.len()]
	}

	fn dp(st: usize, ed: usize, s: &[u8], memo: &mut Vec<Vec<i32>>) {
		if memo[st][ed] != i32::max_value() {
			return;
		}
		if st == ed {
			memo[st][ed] = 0;
			return;
		}
		if Solution::is_palindrome(st, ed, s) {
			memo[st][ed] = 0;
			return
		}
		for i in st+1..ed {
			if Solution::is_palindrome(st, i, s) {
				Solution::dp(i, ed, s, memo);
				let ret = memo[i][ed];
				memo[st][ed] = memo[st][ed].min(1 + ret);
			}
		}
	}

	fn is_palindrome(st: usize, ed: usize, s: &[u8]) -> bool {
		for i in 0..(ed - st) / 2 {
			if s[st + i] != s[ed - 1 - i] {
				return false;
			}
		}
		true
	}
}