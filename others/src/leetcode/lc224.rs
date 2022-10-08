struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "1 + 1";
		assert_eq!(Solution::calculate(s.to_string()), 2);
	}

	#[test]
	fn test2() {
		let s = "2-1 + 2";
		assert_eq!(Solution::calculate(s.to_string()), 3);
	}

	#[test]
	fn test3() {
		let s = "(1+(4+5+2)-3)+(6+8)";
		assert_eq!(Solution::calculate(s.to_string()), 23);

		let s = "()";
		let ans = 0;
		assert_eq!(Solution::calculate(s.to_string()), ans);

		let s = "(1-(1-323))";
		let ans = 323;
		assert_eq!(Solution::calculate(s.to_string()), ans);

		let s = "(100-(1-323))";
		let ans = 422;
		assert_eq!(Solution::calculate(s.to_string()), ans);

		let s = "(100-(1-323)) - 100 + 1000";
		let ans = 1322;
		assert_eq!(Solution::calculate(s.to_string()), ans);

		let s = "23 + (100-(1-323)) - (100 + 1000)";
		let ans = -655;
		assert_eq!(Solution::calculate(s.to_string()), ans);
	}

	#[test]
	fn test4() {
		let s = "- 2123 + 112";
		let ans = -2011;
		assert_eq!(Solution::calculate(s.to_string()), ans);
	}

	#[test]
	fn test5() {
		let s = "- (3 + (4 + 5))";
		let ans = -12;
		assert_eq!(Solution::calculate(s.to_string()), ans);
	}

	#[test]
	fn test6() {
		let s = "1 - (-2)";
		let ans = 3;
		assert_eq!(Solution::calculate(s.to_string()), ans);
	}
}

impl Solution {
	pub fn calculate(s: String) -> i32 {
		let s = s.as_bytes();
		let mut ops = vec![1];
		let mut sign = 1;
		let mut ans = 0;
		let mut i = 0;
		while i < s.len() {
			match s[i] {
				b' ' => { i += 1; }
				b'+' => {
					sign = *ops.last().unwrap();
					i += 1;
				}
				b'-' => {
					sign = -*ops.last().unwrap();
					i += 1;
				}
				b'(' => {
					ops.push(sign);
					i += 1;
				}
				b')' => {
					ops.pop();
					i += 1;
				}
				_ => {
					let mut num = 0;
					while i < s.len() && s[i] >= b'0' && s[i] <= b'9' {
						num = num * 10 + (s[i] - b'0') as i32;
						i += 1;
					}
					ans += sign * num;
				}
			}
		}
		ans
	}
}
