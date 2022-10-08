struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Solution::candy(vec![1,3,2,2,1]), 7);
	}
}

impl Solution {
	pub fn candy(ratings: Vec<i32>) -> i32 {
		let mut ans = 0;
		let mut i = 0;
		let mut pre = 0;
		while i < ratings.len() {
			let mut j = i + 1;
			while j < ratings.len() && ratings[j] < ratings[j-1] {
				j += 1;
			}
			let t = j - i;
			ans += (t + 1) * t / 2;
			if i != 0 {
				if ratings[i] > ratings[i - 1] {
					ans += std::cmp::max(pre + 1, t) - t;
				}
			}
			if t > 1 || i == 0 || ratings[i] == ratings[i - 1] {
				pre = 1;
			} else {
				pre += 1;
			}
			i = j;
		}
		ans as i32
	}
}