struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "ababcbacadefegdehijhklij".to_string();
		assert_eq!(Solution::partition_labels(s), vec![9, 7, 8]);
	}
}

use std::cmp::max;

impl Solution {
	pub fn partition_labels(s: String) -> Vec<i32> {
		let s = s.chars().collect::<Vec<char>>();
		let mut last_pos = vec![0; 256];
		for i in (0..s.len()).rev() {
			let k = s[i] as usize;
			last_pos[k] = max(last_pos[k], i);
		}
		let mut ans = vec![];
		let mut i = 0;
		while i < s.len() {
			let pre = i;
			let mut j = last_pos[s[i] as usize];
			while i < j {
				i += 1;
				j = max(j, last_pos[s[i] as usize]);
			}
			ans.push((j - pre + 1) as i32);
			i = j + 1;
		}
		ans
	}
}