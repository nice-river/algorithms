struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn flip_and_invert_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		for row in a.iter_mut() {
			row.reverse();
			row.iter_mut().for_each(|x| *x ^= 1);
		}
		a
	}
}