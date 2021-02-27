struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
		let n = matrix.len();
		let m = matrix[0].len();
		for i in 0..n {
			for k in 1..n {
				let x = i + k;
				let y = k;
				if x >= n || y >= m {
					break;
				}
				if matrix[x][y] != matrix[i][0] {
					return false;
				}
			}
		}

		for j in 1..m {
			for k in 1..m {
				let x = k;
				let y = j + k;
				if x >= n || y >= m {
					break;
				}
				if matrix[x][y] != matrix[0][j] {
					return false;
				}
			}
		}
		true
	}
}