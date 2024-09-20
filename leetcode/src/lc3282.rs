#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
	let mut ans = 0;
	let mut x = nums[0];
	for num in nums.into_iter().skip(1) {
	    ans += x as i64;
	    x = x.max(num);
	}
	ans
    }
}
