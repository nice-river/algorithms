struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
		let n = nums.len();
		if n == 0 {
			return nums;
		}
		let m = nums[0].len();
		let r = r as usize;
		let c = c as usize;
		if n * m != r * c {
			return nums;
		}
		let mut ans = vec![vec![0; c]; r];
		let mut i = 0;
		for row in nums.iter() {
			for &cell in row {
				ans[i / c][i % c] = cell;
				i += 1;
			}
		}
		ans
	}
}