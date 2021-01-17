struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sol = Solution {};
	}
}

impl Solution {
	pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
		let a = coordinates[1][0] - coordinates[0][0];
		let b = coordinates[1][1] - coordinates[0][1];
		for i in 1..coordinates.len() {
			let x = coordinates[i][0] - coordinates[0][0];
			let y = coordinates[i][1] - coordinates[0][1];
			if a * y != x * b {
				return false;
			}
		}
		true
	}
}