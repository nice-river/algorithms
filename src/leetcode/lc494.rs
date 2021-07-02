struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let nums = vec![1, 1];
        let target = 1;
        let ans = 0;
        assert_eq!(Solution::find_target_sum_ways(nums, target), ans);
	}
}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        const SHIFT: usize = 1000;
        let mut dp = vec![vec![0; SHIFT * 2 + 1]; 2];
        let n = dp[0].len();
        let mut cur = 0;
        dp[cur][SHIFT] = 1;
        for num in nums {
            let num = num as usize;
            let nxt = cur ^ 1;
            dp[nxt].iter_mut().map(|v| *v = 0).count();
            for i in 0..dp[cur].len() {
                if i >= num {
                    dp[nxt][i - num] += dp[cur][i];
                }
                if i + num < n {
                    dp[nxt][i + num] += dp[cur][i];
                }
            }
            cur ^= 1;
        }
        dp[cur][(SHIFT as i32 + target) as usize]
    }
}