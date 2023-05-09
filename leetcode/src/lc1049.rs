struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let stones = vec![31,26,33,21,40,31,26,33,21,40,31,26,33,21,40,31,26,33,21,40,31,26,33,21,40,31,26,33,21,40,31,26,33,21,40];
        let ans = 1;
		assert_eq!(Solution::last_stone_weight_ii(stones), ans);
	}
}

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let s: i32 = stones.iter().sum();
		let m = (s / 2) as usize + 1;
        let mut dp = vec![vec![false; m]; 2];
		let mut cur = 0;
		dp[cur][0] = true;
		for stone in stones {
            let nxt = cur ^ 1;
			for i in 0..m {
				dp[nxt][i] |= dp[cur][i];
                let j = stone as usize + i;
				if j < m {
					dp[nxt][j] |= dp[cur][i];
				}
			}
			cur ^= 1;
		}
        for i in (0..m).rev() {
			if dp[cur][i] {
				return s - i as i32 - i as i32;
			}
		}
		return s;
    }
}