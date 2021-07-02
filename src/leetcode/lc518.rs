struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let amount = 5;
		let coins = vec![1, 2, 5];
		let ans = 4;
		assert_eq!(Solution::change(amount, coins), ans);
	}
}


impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
		let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;
        for coin in coins {
			for i in coin as usize..=amount {
                dp[i] += dp[i - coin as usize];
			}
		}
		dp[amount]
    }
}