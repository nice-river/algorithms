struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = 4;
		let relation = vec![vec![0, 1], vec![1, 3], vec![0, 2], vec![2, 3]];
		let k = 2;
        let ans = 2;
		assert_eq!(Solution::num_ways(n, relation, k), ans);
	}
}

use std::collections::HashMap;

impl Solution {
    pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
		let n = n as usize;
		let mut dp = vec![vec![0; n]; 2];
		dp[0][0] = 1;
		let mut map = HashMap::new();
		for rel in relation.iter() {
			map.entry(rel[0] as usize).or_insert(vec![]).push(rel[1] as usize);
		}

		let mut cur = 0;
		for _ in 0..k {
			let nxt = cur ^ 1;
            for i in 0..n {dp[nxt][i] = 0};
			for i in 0..n {
                if let Some(adj) = map.get(&i) {
					for &j in adj {
						dp[nxt][j] += dp[cur][i];
					}
				}
			}
			cur ^= 1;
		}
        dp[cur][n - 1]
    }
}