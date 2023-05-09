#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
	    let mut ans = nums[0];
	    let mut mini = 0;
	    let mut s = 0;
	    for num in nums {
		    s += num;
		    ans = ans.max(s - mini);
		    mini = mini.min(s);
	    }
	    ans
    }
}