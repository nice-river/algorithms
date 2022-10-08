struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
		let m = matrix[0].len();
        let mut dp = vec![vec![0; m + 1]; n + 1];
		let mut val = Vec::with_capacity(n * m);
		for i in 1..=n {
			for j in 1..=m {
				dp[i][j] = dp[i-1][j] ^ dp[i][j-1] ^ dp[i-1][j-1] ^ matrix[i-1][j-1];
				val.push(dp[i][j]);
			}
		}
		val.sort_unstable_by_key(|v| -v);
		val[k as usize - 1]
    }
}