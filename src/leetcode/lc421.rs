#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![3,10,5,25,2,8];
		let ans = 28;
		assert_eq!(Solution::find_maximum_xor(nums), ans);
	}

	#[test]
	fn test1() {
		let nums = vec![14,70,53,83,49,91,36,80,92,51,66,70];
		let ans = 127;
		assert_eq!(Solution::find_maximum_xor(nums), ans);
	}
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
	    let mut prefix_set = vec![HashSet::new(); 31];
	    let mut ans = 0;
	    for num in nums {
            let mut x = 0;
            for k in (0..31).rev() {
	            x |= ((1 << k) & num) ^ (1 << k);
	            if !prefix_set[k].contains(&x) {
		            x ^= (1 << k);
	            }
            }
            ans = ans.max(num ^ x);
            x = 0;
		    for k in (0..31).rev() {
			    x |= (1 << k) & num;
                prefix_set[k].insert(x);
		    }
	    }
	    ans
    }
}