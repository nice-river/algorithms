struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let tokens = ["1","2","3","+","-"];
		let tokens = tokens.to_vec().into_iter().map(|s| s.to_string()).collect();
		assert_eq!(Solution::eval_rpn(tokens), -4);
	}
}


impl Solution {
	pub fn eval_rpn(tokens: Vec<String>) -> i32 {
		let mut stk: Vec<i32> = vec![];

		for token in tokens {
			match token.as_bytes() {
				b"+" => {
					let r = stk.pop().unwrap() + stk.pop().unwrap();
					stk.push(r);
				}
				b"-" => {
					let r = -stk.pop().unwrap() + stk.pop().unwrap();
					stk.push(r);
				}
				b"*" => {
					let r = stk.pop().unwrap() * stk.pop().unwrap();
					stk.push(r);
				}
				b"/" => {
					let a = stk.pop().unwrap();
					let b = stk.pop().unwrap();
					stk.push(b / a);
				}
				_ => {
					stk.push(token.parse::<i32>().unwrap());
				}
			}
		}
		stk[0]
	}
}