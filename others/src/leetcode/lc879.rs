use std::cmp::min;

struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = 5;
		let min_profit = 3;
		let group = vec![2, 2];
		let profit = vec![2, 3];
		let ans = 2;
		assert_eq!(Solution::profitable_schemes(n, min_profit, group, profit), ans);
	}

	#[test]
	fn test1() {
		let n = 1;
		let min_profit = 1;
		let group = vec![1];
		let profit = vec![2];
		let ans = 1;
		assert_eq!(Solution::profitable_schemes(n, min_profit, group, profit), ans);
	}
}


impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
		const MOD: usize = 1_000_000_007;
		let n = n as usize;
		let min_profit = min_profit as usize;
        let mut dp = vec![vec![vec![0; min_profit + 1]; n + 1]; 2];
		let mut cur = 0;
		dp[cur][0][0] = 1;
		for i in 0..group.len() {
			let nxt = cur ^ 1;
            dp[nxt] = dp[cur].clone();
            for j in 0..=n {
				let x = j + group[i] as usize;
				if x > n {
                    break;
				}
				for k in 0..=min_profit {
                    let y = (k + profit[i] as usize).min(min_profit);
                    dp[nxt][x][y] += dp[cur][j][k];
					dp[nxt][x][y] %= MOD;
				}
			}
			cur ^= 1;
		}
		let mut ans = 0;
        for arr in dp[cur].iter() {
			ans += arr[min_profit];
			ans %= MOD;
		}
        ans as i32
    }
}