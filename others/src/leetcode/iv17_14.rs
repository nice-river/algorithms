struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn smallest_k(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
		arr.sort_unstable();
		arr[0..k as usize].iter().cloned().collect()
	}
}