struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let a = vec![1,1,1,0,0,0,1,1,1,1,0];
		let k = 2;
		assert_eq!(Solution::longest_ones(a, k), 6);

		let a = vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1];
		let k = 3;
		assert_eq!(Solution::longest_ones(a, k), 10);
	}
}

impl Solution {
	pub fn longest_ones(a: Vec<i32>, mut k: i32) -> i32 {
		let mut i = 0;
		let mut j = 0;
		while j < a.len() {
			if a[j] == 0 {
				k -= 1;
			}
			if k < 0 {
				k += if a[i] == 0 { 1 } else { 0 };
				i += 1;
			}
			j += 1;
		}
		(j - i) as i32
	}
}