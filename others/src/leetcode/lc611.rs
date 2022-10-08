struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![2, 2, 3, 4];
		let ans = 3;
		assert_eq!(Solution::triangle_number(nums), ans);
	}

	#[test]
	fn test1() {
		let nums = vec![0, 0, 0];
		let ans = 0;
		assert_eq!(Solution::triangle_number(nums), ans);
	}

	#[test]
	fn test2() {
		let nums = vec![1, 2, 2];
		let ans = 1;
		assert_eq!(Solution::triangle_number(nums), ans);
	}
}


impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
		let n = nums.len();
		nums.sort_unstable();
		let mut ans = 0;
		for i in 0..n {
			let mut k = i + 2;
			for j in i+1..n {
				k = k.max(j + 1);
				while k < n && nums[k] < nums[i] + nums[j] {
					k += 1;
				}
				ans += k - (j + 1);
			}
		}
		ans as i32
    }
}