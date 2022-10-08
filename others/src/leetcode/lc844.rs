struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn backspace_compare(s: String, t: String) -> bool {
		let mut a = vec![];
		let mut b = vec![];
		Solution::construct(&s, &mut a);
		Solution::construct(&t, &mut b);
		a == b
	}

	fn construct(s: &String, v: &mut Vec<char>) {
		for ch in s.chars() {
			if ch == '#' {
				v.pop();
			} else {
				v.push(ch);
			}
		}
	}
}