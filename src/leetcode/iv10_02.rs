#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
	    let mut map = HashMap::new();
	    for (i, str) in strs.iter().enumerate() {
		    let mut k = str.clone();
		    unsafe { k.as_bytes_mut() }.sort_unstable();
		    map.entry(k).or_insert(vec![]).push(i);
	    }
	    let mut ans = Vec::with_capacity(map.len());

	    for (_, arr) in map.into_iter() {
		    let mut s = Vec::with_capacity(arr.len());
		    for idx in arr {
			    s.push(strs[idx].clone());
		    }
		    ans.push(s);
	    }
	    ans
    }
}