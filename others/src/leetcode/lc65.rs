struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let valid = ["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789", "-.1", ".1", "+.1"];
        for &s in &valid {
			assert!(Solution::is_number(s.to_string()));
		}
	}

	#[test]
	fn test1() {
		let invalid = ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53", "3.e0.1", "3.e+", "+.", "-e10", "e10"];
		for &s in &invalid {
			assert!(!Solution::is_number(s.to_string()));
		}
	}
}

enum TokenState {
    Symbol,
    FirstNumber,
	Number,
	FirstFraction,
	Fraction,
	FirstExponential,
    FirstExpInteger,
	ExpInteger,
}


impl Solution {
	pub fn is_number(s: String) -> bool {
		use TokenState::*;
		let s = s.as_bytes();
		let mut state = Symbol;
		for ch in s {
			match state {
				Symbol => {
					match ch {
						b'+' | b'-' => state = FirstNumber,
						b'0'..=b'9' => state = Number,
						b'.' => state = FirstFraction,
						_ => return false,
					}
				}
				FirstNumber => {
					match ch {
						b'0'..=b'9' => state = Number,
						b'.' => state = FirstFraction,
						_ => return false,
					}
				}
				Number => {
					match ch {
						b'0'..=b'9' => state = Number,
						b'.' => state = Fraction,
                        b'e' | b'E' => state = FirstExponential,
						_ => return false,
					}
				}
				FirstFraction => {
					match ch {
						b'0'..=b'9' => state = Fraction,
						_ => return false,
					}
				}
				Fraction => {
					match ch {
						b'0'..=b'9' => state = Fraction,
						b'e' | b'E' => state = FirstExponential,
						_ => return false,
					}
				}
				FirstExponential => {
					match ch {
						b'+' | b'-' => state = FirstExpInteger,
						b'0'..=b'9' => state = ExpInteger,
						_ => return false,
					}
				}
				FirstExpInteger => {
					match ch {
						b'0'..=b'9' => state = ExpInteger,
						_ => return false,
					}
				}
				ExpInteger => {
					match ch {
						b'0'..=b'9' => state = ExpInteger,
						_ => return false,
					}
				}
			}
		}
        match state {
			Symbol | FirstNumber | FirstFraction | FirstExponential | FirstExpInteger => false,
			_ => true,
		}
	}
}