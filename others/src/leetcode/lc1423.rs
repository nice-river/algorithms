struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let card_points = [1, 2, 3, 4, 5, 6, 1];
		let k = 3;
		assert_eq!(Solution::max_score(card_points.to_vec(), k), 12);
	}
}

impl Solution {
	pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
		let k = k as usize;
		let mut ans: i32 = card_points[0..k].iter().sum();
		let mut cur = ans;
		for i in 1..=k {
			cur -= card_points[k - i];
			cur += card_points[card_points.len() - i];
			ans = ans.max(cur);
		}
		ans
	}
}