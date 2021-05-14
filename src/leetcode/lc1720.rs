struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
		let mut ans = vec![first; encoded.len() + 1];
        for i in 0..encoded.len() {
            ans[i + 1] = ans[i] ^ encoded[i];
		}
		ans
	}
}