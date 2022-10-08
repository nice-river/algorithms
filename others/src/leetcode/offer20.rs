struct Helper {}

impl Solution {
	pub fn is_number(s: String) -> bool {
		let s = s.trim();
		if let Ok(_) = s.parse::<f64>() {
			true
		} else {
			false
		}
	}
}

pub struct Solution {}