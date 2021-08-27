struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut nums = vec![2, 0, 2, 1, 1, 0];
		Solution::sort_colors(&mut nums);
		assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
	}
}

impl Solution {
	pub fn sort_colors(nums: &mut Vec<i32>) {
		let n = nums.len();
		let (mut x, mut y) = (0, n - 1);
		while x < n && nums[x] == 0 {
			x += 1;
		}
		while y + 1 != 0 && nums[y] == 2 {
			y -= 1;
		}
		if y + 1 == 0 {
			return ;
		}
		let mut i = x;
		while i <= y {
			match nums[i] {
				0 => {
					if x >= i {
						while nums[x] == 0 {
							x += 1;
						}
						i = x;
						continue ;
					}
					nums[i] = nums[x];
					nums[x] = 0;
					x += 1;
				}
				2 => {
					nums[i] = nums[y];
					nums[y] = 2;
					y -= 1;
				}
				_ => {
					i += 1;
				}
			}
		}
	}
}