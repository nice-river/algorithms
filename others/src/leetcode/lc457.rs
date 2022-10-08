#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![2,-1,1,2,2];
		assert!(Solution::circular_array_loop(nums));
	}

	#[test]
	fn test1() {
		let nums = vec![-1,2];
		assert!(!Solution::circular_array_loop(nums));
	}

	#[test]
	fn test2() {
		let nums = vec![-2,1,-1,-2,-2];
		assert!(!Solution::circular_array_loop(nums));
	}


	#[test]
	fn test3() {
		let nums = vec![1,1,1,1,-5,-1];
		assert!(Solution::circular_array_loop(nums));
	}
}

struct Solution {}

impl Solution {
	pub fn circular_array_loop(mut nums: Vec<i32>) -> bool {
		let base = 10000;
		let n = nums.len();
		for i in 0..nums.len() {
			if nums[i] < 0 || nums[i] >= base {
				continue;
			}
			let mut pre = i;
			let mut cur = (i + nums[i] as usize) % n;
			nums[pre] = base + i as i32;
			while nums[cur] > 0 && nums[cur] < base {
				pre = cur;
				cur = (cur + nums[cur] as usize) % n;
				nums[pre] = base + i as i32;
			}
			if nums[cur] > 0 && cur != pre && nums[cur] == nums[pre] {
				return true;
			}
		}
		for i in 0..nums.len() {
			if nums[i] > 0 || nums[i] <= -base {
				continue;
			}
			let mut pre = i;
			let mut cur = (i as i32 + nums[i] + n as i32) as usize % n;
			nums[pre] = -base - i as i32;
			while nums[cur] < 0 && nums[cur] > -base {
				pre = cur;
				cur = (cur as i32 + nums[cur] + n as i32) as usize % n;
				nums[pre] = -base - i as i32;
			}
			if nums[cur] < 0 && cur != pre  && nums[cur] == nums[pre] {
				return true;
			}
		}
		false
	}
}