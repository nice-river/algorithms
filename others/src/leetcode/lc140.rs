
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let word_dict = ["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
		let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
		let ans = Solution::word_break(s, word_dict);
		println!("{:?}", ans);
		// assert_eq!(Solution::word_break(s, word_dict), ans);
	}
}

struct Solution {}

impl Solution {
	pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
		let s = s.into_bytes();
		let n = s.len();
		let word_dict = word_dict.into_iter().map(|g| g.into_bytes()).collect::<Vec<Vec<u8>>>();
		let mut memo = vec![None; n];
		Solution::dfs(0, &mut memo, &s, &word_dict);

		let mut ans = vec![];
		Solution::construct(0, &mut vec![], &mut ans, &s, &memo);
		ans
	}

	fn dfs(pos: usize, memo: &mut Vec<Option<Vec<usize>>>, s: &Vec<u8>, word_dict: &Vec<Vec<u8>>) {
		let n = s.len();
		if memo[pos].is_some() {
			return ;
		}
		for word in word_dict.iter() {
			let k = word.len();
			if pos + k > n {
				continue ;
			}
			if s[pos..pos+k] == word[..] {
				if pos + k == n {
					if let Some(m) = &mut memo[pos] {
						m.push(pos + k);
					} else {
						memo[pos] = Some(vec![pos + k]);
					}
				} else {
					Solution::dfs(pos + k, memo, s, word_dict);
					let (left, right) = memo.split_at_mut(pos + k);
					if let Some(r) = &right[0] {
						if r.len() != 0 {
							if let Some(m) = &mut left[pos] {
								m.push(pos + k);
							} else {
								memo[pos] = Some(vec![pos + k]);
							}
						}
					}
				}
			}
		}
		if memo[pos].is_none() {
			memo[pos] = Some(vec![]);
		}
	}

	fn construct(pos: usize, cur: &mut Vec<String>, ans: &mut Vec<String>, s: &Vec<u8>, memo: &Vec<Option<Vec<usize>>>) {
		if pos == memo.len() {
			ans.push(cur.join(" "));
			return ;
		}
		if let Some(arr) = &memo[pos] {
			for nxt in arr {
				cur.push(std::str::from_utf8(&s[pos..*nxt]).unwrap().to_string());
				Solution::construct(*nxt, cur, ans, s, memo);
				cur.pop();
			}
		}
	}
}