#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = "MMCCCVII";
		assert_eq!(Solution::roman_to_int(s.to_string()), 2307);
	}
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut map = HashMap::new();
	    map.insert(b'I', 1);
	    map.insert(b'V', 5);
	    map.insert(b'X', 10);
	    map.insert(b'L', 50);
	    map.insert(b'C', 100);
	    map.insert(b'D', 500);
	    map.insert(b'M', 1000);
	    let s = s.as_bytes();
	    let mut ans = 0;
        for i in 0..s.len() {
	        if i + 1 < s.len() && map[&s[i]] < map[&s[i + 1]] {
		        ans -= map[&s[i]];
	        } else {
		        ans += map[&s[i]];
	        }
        }
	    ans
    }
}