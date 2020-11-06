struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
		let mut arr = arr.clone();
		arr.sort_by(|a, b| {
			let x = a.count_ones();
			let y = b.count_ones();
			if x == y {
				a.partial_cmp(b).unwrap()
			} else {
				x.partial_cmp(&y).unwrap()
			}
		});
		arr
	}
}