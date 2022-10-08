struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = 100;
		let ans = 21;
		assert_eq!(Solution::count_digit_one(n), ans);
	}
}


impl Solution {
	pub fn count_digit_one(mut n: i32) -> i32 {
		if n <= 1 {
			return n;
		}
		let mut digits = Vec::new();
		while n != 0 {
			digits.push(n % 10);
			n /= 10;
		}
		digits.reverse();
		let mut dp = vec![None; digits.len()];
		Solution::dfs(0, &digits, false, &mut dp).0
	}

	fn dfs(idx: usize, digits: &Vec<i32>, is_smaller: bool, dp: &mut Vec<Option<(i32, i32)>>) -> (i32, i32) {
		let maxi = if is_smaller { 9 } else { digits[idx] };
		if idx + 1 == digits.len() {
			return (if maxi >= 1 {1} else {0}, maxi + 1);
		}
		if is_smaller && dp[idx].is_some() {
			return dp[idx].clone().unwrap();
		}
		let mut ones = 0;
		let mut cnt = 0;
		for i in 0..=maxi {
			let (x, y) =  Solution::dfs(idx + 1, digits, is_smaller | (i < maxi), dp);
			ones += x;
			if i == 1 {
				ones += y;
			}
			cnt += y;
		}
		if is_smaller {
			dp[idx] = Some((ones, cnt));
		}
		(ones, cnt)
	}
}