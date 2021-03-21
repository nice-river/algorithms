#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
	pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
		let n = matrix.len();
		let m = matrix[0].len();
		let mut flag_col0 = false;
		for i in 0..n {
			if matrix[i][0] == 0 {
				flag_col0 = true;
			}
			for j in 1..m {
				if matrix[i][j] == 0 {
					matrix[i][0] = 0;
					matrix[0][j] = 0;
				}
			}
		}
		for i in (0..n).rev() {
			for j in 1..m {
				if matrix[i][0] == 0 || matrix[0][j] == 0 {
					matrix[i][j] = 0;
				}
			}
			if flag_col0 {
				matrix[i][0] = 0;
			}
		}
	}
}