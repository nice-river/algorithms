#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
	pub fn fib(n: i32) -> i32 {
		if n <= 1 {
			return n;
		}
		let module = 10i64.pow(9) + 7;
		let mut a = 0;
		let mut b = 1;
		let mut c = 0;
		for i in 2..=n {
			c = (a + b) % module;
			a = b;
			b = c;
		}
		c as i32
	}
}