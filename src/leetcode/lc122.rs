#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}


impl Solution {
	pub fn max_profit(prices: Vec<i32>) -> i32 {
		let mut p = prices[0];
		let mut ans = 0;
		for i in 0..prices.len()-1 {
			if prices[i] > prices[i + 1] {
				ans += prices[i] - p;
				p = prices[i + 1];
			}
		}
		ans += prices.last().unwrap() - p;
		ans
	}
}