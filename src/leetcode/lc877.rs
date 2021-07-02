struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let piles = vec![5,3,4,5];
        assert!(Solution::stone_game(piles));
	}
}

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
		let n = piles.len();
		let mut pre_sum = vec![0; n + 1];
		for i in 0..n {
			pre_sum[i+1] = pre_sum[i] + piles[i];
		}
        let mut dp = vec![vec![0; n]; n + 1];
		for i in 0..n {
			dp[1][i] = piles[i];
		}
		for i in 2..=n {
			for j in 0..n-i+1 {
				dp[i][j] = (piles[j] + pre_sum[j+i] - pre_sum[j+1] - dp[i-1][j+1]).max(piles[j+i-1] + pre_sum[j+i-1] - pre_sum[j] - dp[i-1][j]);
			}
		}
        dp[n][0] > pre_sum[n] - dp[n][0]
    }
}