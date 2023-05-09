#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

use std::iter::repeat;
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
	    let mut map = HashMap::new();
	    for ch in s.chars() {
		    *map.entry(ch).or_insert(0) += 1;
	    }
	    let mut arr = map.into_iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
	    arr.sort_unstable();
	    arr.reverse();
	    arr.into_iter().map(|(cnt, ch)| repeat(ch).take(cnt).collect::<String>()).collect::<Vec<_>>().join("")
    }
}