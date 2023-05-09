struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn unique_paths(m: i32, n: i32) -> i32 {
		let m = m as usize;
		let n = n as usize;
		let mut memo = vec![vec![1; m]; n];
		for i in 1..n {
			for j in 1..m {
				memo[i][j] = memo[i][j-1] + memo[i-1][j];
			}
		}
		memo[n-1][m-1]
	}
}