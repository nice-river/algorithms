struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
		let mut map = HashMap::new();
		let mut ans: i64 = 0;
		let m = 10i64.pow(9) + 7;
        for d in deliciousness {
            for i in 0..=21 {
                if let Some(&x) = map.get(&((1 << i) - d)) {
                    ans = (ans + x) % m;
				}
			}
            *map.entry(d).or_insert(0) += 1;
		}
		ans as i32
    }
}