struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let nums = vec![1,1,0,0];
        assert_eq!(Solution::find_max_length(nums), 4);
	}
}


use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        map.insert(1, -1);
        let mut ans = 0;
        let mut su = 0;
        for (i, num) in nums.into_iter().enumerate() {
            let i = i as i32;
            su += num;
            let k = su * 2 - i;
            if let Some(&p) = map.get(&k) {
                ans = ans.max(i - p);
            } else {
                map.insert(k, i);
            }
        }
        ans
    }
}