#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
	let mut ans = vec![];
	let mut set = HashSet::new();
	for num in nums {
	    if !set.insert(num) {
		ans.push(num);
	    }
	}
	ans
    }
}
