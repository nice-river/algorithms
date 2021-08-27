#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

use std::collections::BTreeMap;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
	    let mut map = BTreeMap::new();
	    for num in nums {
            *map.entry(num).or_insert(0) += 1;
	    }
	    let mut a = 0;
	    let mut b = 0;
	    let mut pre = i32::MIN;
	    for (k, v) in map.into_iter() {
            if pre + 1 == k {
	            let c = a;
                a = b + k * v;
	            b = c.max(b);
            } else {
	            let c = a.max(b);
	            a = c + k * v;
	            b = c;
            }
		    pre = k;
	    }
        a.max(b)
    }
}