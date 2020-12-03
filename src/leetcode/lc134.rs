struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let gas = vec![1, 2, 3, 4, 5];
		let cost = vec![3, 4, 5, 1, 2];
		assert_eq!(Solution::can_complete_circuit(gas, cost), 3);
	}
}

impl Solution {
	pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
		let mut ans = 0;
		let (mut tot, mut cur) = (0, 0);
		for i in 0..gas.len(){
			tot += gas[i] - cost[i];
			cur += gas[i] - cost[i];
			if cur < 0 {
				ans = i as i32 + 1;
				cur = 0;
			}
		}
		if tot >= 0 {
			ans
		} else {
			-1
		}
	}
}