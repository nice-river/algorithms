struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn max_profit(prices: Vec<i32>) -> i32 {
		if prices.len() < 2 {
			return 0;
		}
		let n = prices.len();
		let k = std::cmp::min(3, n + 1);
		let mut memo = vec![vec![vec![std::i32::MIN; k]; n]; 2];
		memo[0][0][0] = 0;
		memo[1][0][0] = -prices[0];
		for i in 1..n {
			for j in 0..k {
				memo[0][i][j] = memo[0][i-1][j];
				if j > 0 && memo[1][i-1][j-1] != std::i32::MIN {
					memo[0][i][j] = std::cmp::max(memo[1][i-1][j-1] + prices[i], memo[0][i-1][j]);
				}
				memo[1][i][j] = memo[1][i-1][j];
				if memo[0][i-1][j] != std::i32::MIN {
					memo[1][i][j] = std::cmp::max(memo[1][i - 1][j], memo[0][i - 1][j] - prices[i]);
				}
				// println!("{} {} {} {}", i, j, memo[0][i][j], memo[1][i][j]);
			}
		}
		*memo[0][n-1].iter().max().unwrap()
	}
}