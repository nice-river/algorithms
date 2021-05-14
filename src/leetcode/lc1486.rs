struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
		(0..n).enumerate().map(|(idx, num)| start + 2 * idx as i32).fold(0, |a, b| a ^ b)
    }
}