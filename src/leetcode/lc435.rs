struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
		if intervals.is_empty() {
			return 0;
		}
		intervals.sort_by_key(|v| (v[1], v[0]));
		let mut ans = 0;
		let mut k = intervals[0][1];
		for i in 1..intervals.len() {
			if intervals[i][0] < k {
				ans += 1;
			} else {
				k = intervals[i][1];
			}
		}
		ans
	}
}