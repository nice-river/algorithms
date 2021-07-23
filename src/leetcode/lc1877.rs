struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
		let n = nums.len();
        nums.sort_unstable();
		let mut ans = 0;
        for i in 0..n/2 {
            ans = ans.max(nums[i] + nums[n - 1 - i]);
		}
		ans
    }
}