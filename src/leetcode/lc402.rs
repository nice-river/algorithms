struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let num = "1432219".to_string();
		let k = 3;
		assert_eq!(Solution::remove_kdigits(num, k), "1219".to_string());
	}
}

impl Solution {
	pub fn remove_kdigits(num: String, k: i32) -> String {
		let num = num.as_bytes();
		let n = num.len();
		let mut rem = num.len() - k as usize;
		let mut i = 0;
		let mut ans = vec![];
		let mut is_leading_zero = true;
		while rem > 0 {
			for j in i..n - rem + 1{
				if num[j] < num[i] {
					i = j;
				}
			}
			if !is_leading_zero || num[i] != '0' as u8 {
				ans.push(num[i]);
				is_leading_zero = false;
			}
			i += 1;
			rem -= 1;
		}
		if ans.is_empty() {
			ans.push('0' as u8);
		}
		std::str::from_utf8(ans.as_slice()).unwrap().to_string()
	}
}