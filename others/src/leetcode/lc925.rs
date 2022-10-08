struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let name = "alex".to_string();
		let typed = "aaleex".to_string();
		assert!(Solution::is_long_pressed_name(name, typed));
	}
}

impl Solution {
	pub fn is_long_pressed_name(name: String, typed: String) -> bool {
		let name = name.chars().collect::<Vec<char>>();
		let typed = typed.chars().collect::<Vec<char>>();
		let mut i = 0;
		let mut j = 0;
		for ch in name {
			if j >= typed.len() {
				return false;
			}
			if typed[j] != ch {
				while j != 0 && j < typed.len() && typed[j] == typed[j - 1] {
					j += 1;
				}
				if j == typed.len() || typed[j] != ch {
					return false;
				}
			}
			j += 1;
		}
		while j != 0 && j < typed.len() && typed[j] == typed[j - 1] {
			j += 1;
		}
		j >= typed.len()
	}
}