struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


impl Solution {
	pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
		let n = matrix.len();
		let m = matrix[0].len();
		let mut i = 0;
		let mut j = m - 1;
		while i < n {
			if matrix[i][j] == target {
				return true;
			} else if matrix[i][j] < target {
				i += 1;
			} else if j == 0 {
				break;
			} else {
				j -= 1;
			}
		}
		false
	}
}