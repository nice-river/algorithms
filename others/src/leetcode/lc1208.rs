struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "abcd".to_string();
		let t = "bcdf".to_string();
		let cost = 3;
		let ans = 3;
		assert_eq!(Solution::equal_substring(s, t, cost), ans);
	}
}

macro_rules! diff {
    ($a:expr, $b:expr) => {($a as i32 - $b as i32).abs()};
}

impl Solution {
	pub fn equal_substring(s: String, t: String, mut max_cost: i32) -> i32 {
		let s = s.as_bytes();
		let t = t.as_bytes();
		let mut ans = 0;
		let mut i = 0;
		let mut j = 0;

		while j < s.len() {
			while j < s.len() && diff!(s[j], t[j]) <= max_cost {
				max_cost -= diff!(s[j], t[j]);
				j += 1;
			}
			if i == j {
				i += 1;
				j += 1;
			}
			ans = ans.max((j - i) as i32);
			while j < s.len() && i < j && diff!(s[j], t[j]) > max_cost {
				max_cost += diff!(s[i], t[i]);
				i += 1;
			}
		}
		ans
	}
}