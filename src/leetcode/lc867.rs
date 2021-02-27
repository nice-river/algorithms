struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let n = matrix.len();
		let m = matrix[0].len();
		let mut ans = vec![vec![0; n]; m];
		for i in 0..n {
			for j in 0..m {
				ans[j][i] = matrix[i][j];
			}
		}
		ans
	}
}