#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "aab";
		let ans = vec![vec!["aa","b"], vec!["a","a","b"]];
		let ans: Vec<Vec<String>> = ans.into_iter().map(
			|v| v.into_iter().map(|s| s.to_string()).collect()
		).collect();
		assert_eq!(Solution::partition(s.to_string()), ans);
	}
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
	pub fn partition(s: String) -> Vec<Vec<String>> {
		let s = s.as_bytes();
		let mut memo = HashMap::new();
		Solution::dp(0, s.len(), s, &mut memo);
		let ans = memo.get(&(0, s.len())).unwrap().clone();
		ans.into_iter().map(|mut v| {
			v.reverse();
			v.into_iter().map(|x| std::str::from_utf8(x).unwrap().to_string()).collect()
		}).collect()
	}

	fn dp<'a>(st: usize, ed: usize, s: &'a [u8], memo: &mut HashMap<(usize, usize), Vec<Vec<&'a [u8]>>>) {
		if memo.contains_key(&(st, ed)) {
			return;
		}
		let mut entry = vec![];
		for i in st+1..=ed {
			if Solution::is_palindrome(st, i, s) {
				Solution::dp(i, ed, s, memo);
				let ret = memo.get(&(i, ed)).unwrap();
				if ret.is_empty() {
					entry.push(vec![&s[st..i]]);
				} else {
					for v in ret {
						entry.push(v.clone());
						entry.last_mut().unwrap().push(&s[st..i]);
					}
				}
			}
		}
		memo.insert((st, ed), entry);
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