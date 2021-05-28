struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
		let s = s.as_bytes();
        let mut ans = Vec::with_capacity(s.len());
		let mut stk = Vec::with_capacity(s.len());
        for &ch in s {
			match ch {
				b'(' => stk.push(ans.len()),
				b')' => {
					let i = stk.pop().unwrap();
                    ans[i..].reverse()
				}
				_ => ans.push(ch),
			}
		}
        String::from_utf8(ans).unwrap()
    }
}