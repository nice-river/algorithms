struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let matrix = vec![vec!['1', '1']];
		assert_eq!(Solution::maximal_rectangle(matrix), 2);
	}
}

impl Solution {
	pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
		if matrix.is_empty() {
			return 0;
		}
		let n = matrix.len();
		let m = matrix[0].len();
		let mut up = vec![vec![0; m + 1]; n + 1];
		let mut ans = 0;
		for i in 1..=n {
			for j in 1..=m {
				if matrix[i-1][j-1] == '0' {
					up[i][j] = 0;
				} else {
					up[i][j] = up[i-1][j] + 1;
					let mut k = j;
					let mut t = up[i][j];
					while t != 0 && k != 0 {
						ans = std::cmp::max(ans, t * (j - k + 1) as i32);
						k -= 1;
						t = std::cmp::min(t, up[i][k]);
					}
				}
			}
		}
		ans
	}
}