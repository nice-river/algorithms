struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
		let k = k as usize;
		let mut ans = f64::MIN;
		let mut sum = 0.0f64;
		for i in 0..k - 1 {
			sum += nums[i] as f64;
		}
		for i in k - 1..nums.len() {
			sum += nums[i] as f64;
			ans = ans.max(sum / k as f64);
			sum -= nums[i + 1 - k] as f64;
		}
		ans
	}
}