struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
		let mut memo = vec![std::i32::MAX; cost.len() + 1];
		memo[0] = 0;
		memo[1] = 0;
		for i in 2..memo.len() {
			memo[i] = std::cmp::min(memo[i-2] + cost[i-2], memo[i-1] + cost[i-1]);
		}
		memo[memo.len() - 1]
	}
}