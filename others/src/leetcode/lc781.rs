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
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
	    for answer in answers {
		    *counter.entry(answer).or_insert(0) += 1;
	    }
	    let mut ans = 0;
	    for (k, v) in counter {
            let c = v / (k + 1) + if v % (k + 1) == 0 { 0 } else { 1 };
            ans += c * (k + 1);
	    }
        ans
    }
}