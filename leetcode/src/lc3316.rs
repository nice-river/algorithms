#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

use std::cmp::Reverse;
use std::collections::HashSet;

impl Solution {
    pub fn max_removals(source: String, pattern: String, target_indices: Vec<i32>) -> i32 {
	let source = source.as_bytes();
	let pattern = pattern.as_bytes();
	let n = source.len();
	let m = pattern.len();
	let target_indices: HashSet<i32> = HashSet::from_iter(target_indices);
	let mut dp = vec![vec![(0, Reverse(0)); m + 1]; n + 1];
	for i in 1..=n {
	    for j in 1..=m {
		dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
		if source[i - 1] == pattern[j - 1] {
		    let (a, Reverse(b)) = dp[i - 1][j - 1];
		    dp[i][j] = dp[i][j].max((a + 1, Reverse(if target_indices.contains(&(i as i32 - 1)) { b + 1 } else { b })));
		}
	    }
	}
	target_indices.len() as i32 - dp[n][m].1.0
    }
}
