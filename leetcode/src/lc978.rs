struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let arr = [9,4,2,10,7,8,8,1,9];
		let ans = 5;
		assert_eq!(Solution::max_turbulence_size(arr.to_vec()), ans);
	}
}

impl Solution {
	pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
		let mut i = 0;
		let mut ans = 1;
		while i < arr.len() {
			let mut j = i + 1;
			while j < arr.len() {
				if arr[j-1] == arr[j] {
					break;
				}
				if j >= i + 2 {
					if arr[j] > arr[j-1] && arr[j-1] > arr[j-2] {
						break;
					}
					if arr[j] < arr[j-1] && arr[j-1] < arr[j-2] {
						break;
					}
				}
				j += 1;
			}
			ans = ans.max((j - i) as i32);
			if j != arr.len() && j - i > 1 {
				i = j - 1;
			} else {
				i = j;
			}
		}
		ans
	}
}