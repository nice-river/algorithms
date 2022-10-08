struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let s = "aaabbb";
		let ans = vec!["ababab","ababba","abaabb","abbaba","abbaab","abbbaa","aababb","aabbab","aabbba","aaabbb","bababa","babaab","babbaa","baabab","baabba","baaabb","bbabaa","bbaaba","bbaaab","bbbaaa"];
		let mut ans = ans.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
		let mut ret = Solution::permutation(s.to_string());
		ans.sort_unstable();
		ret.sort_unstable();
		assert_eq!(ret, ans);
	}
}

use std::collections::HashMap;

impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
		let n = s.len();
        let mut map = HashMap::new();
        for ch in s.chars() {
			*map.entry(ch).or_insert(0) += 1;
		}
		let mut map = map.into_iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();
		let mut ans = vec![];
		Solution::dfs(n, &mut map, &mut vec![], &mut ans);
		ans
    }

	fn dfs(n: usize, map: &mut Vec<(char, i32)>, cur: &mut Vec<char>, ans: &mut Vec<String>) {
        if cur.len() == n {
			ans.push(cur.iter().collect());
			return;
		}
        for i in 0..map.len() {
			if map[i].1 > 0 {
                map[i].1 -= 1;
                cur.push(map[i].0);
				Solution::dfs(n, map, cur, ans);
				cur.pop();
				map[i].1 += 1;
			}
		}
	}
}