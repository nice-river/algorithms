struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn fib(n: i32) -> i32 {
		if n < 2 {
			return if n == 0 { 0 } else { 1 };
		}
		let mut f0 = 0;
		let mut f1 = 1;
		let mut ans = 0;
		for _ in 2..=n {
			ans = f0 + f1;
			f0 = f1;
			f1 = ans;
		}
		ans
	}
}