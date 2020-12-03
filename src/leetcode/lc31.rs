struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut v = vec![1, 2, 4, 3];
		Solution::next_permutation(&mut v);
		println!("{:?}", v);
	}
}

impl Solution {
	pub fn next_permutation(nums: &mut Vec<i32>) {
		if nums.len() == 0 {
			return ;
		}
		let mut i = nums.len() - 1;;
		while i > 0 {
			if nums[i] > nums[i - 1] {
				let mut k = i;
				for j in i..nums.len() {
					if nums[j] > nums[i - 1] && nums[j] < nums[k] {
						k = j;
					}
				}
				nums.swap(k, i - 1);
				break;
			}
			i -= 1;
		}
		nums[i..].sort();
	}
}

