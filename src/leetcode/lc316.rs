struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "bbcaac".to_string();
		assert_eq!(Solution::remove_duplicate_letters(s), "bac".to_string());
	}
}

impl Solution {
	pub fn remove_duplicate_letters(s: String) -> String {
		let mut vis = vec![false; 256];
		let mut cnt = vec![0; 256];
		let s = s.into_bytes();
		for &c in s.iter() { cnt[c as usize] += 1};
		let mut stk = Vec::with_capacity(30);
		for i in 0..s.len() {
			if !vis[s[i] as usize] {
				while !stk.is_empty() {
					let &t = stk.last().unwrap();
					if t > s[i] && cnt[t as usize] > 1 {
						cnt[t as usize] -= 1;
						vis[t as usize] = false;
						stk.pop();
					} else {
						break;
					}
				}
				stk.push(s[i]);
				vis[s[i] as usize] = true;
			} else {
				cnt[s[i] as usize] -= 1;
			}
		}
		std::str::from_utf8(&stk).unwrap().to_string()
	}
}
