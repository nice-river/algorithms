struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "abcdefg".to_string();
		let k = 2;
		let ans = "bacdfeg".to_string();
		assert_eq!(Solution::reverse_str(s, k), ans);
	}
}

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
			let k = k as usize;
			let mut ans = s.as_bytes().to_vec();
			let n = ans.len();
			for i in (0..n).step_by(2 * k) {
				if i + k <= n {
					ans[i..i+k].reverse();
				} else {
					ans[i..n].reverse();
				}
			}

			String::from_utf8(ans).unwrap()
    }
}