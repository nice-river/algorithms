#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
	    let mut cnt = vec![0; nums.len() + 1];
	    for num in nums {
		    cnt[num as usize] += 1;
	    }
	    let mut lost = 0;
	    let mut dup = 0;
	    for i in 1..cnt.len() {
		    if cnt[i] == 0 {
			    lost = i as i32;
		    } else if cnt[i] == 2 {
			    dup = i as i32;
		    }
	    }
	    vec![dup, lost]
    }
}