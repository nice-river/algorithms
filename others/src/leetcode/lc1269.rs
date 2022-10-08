struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let steps = 4;
        let arr_len = 2;
        let ans = 8;
        assert_eq!(Solution::num_ways(steps, arr_len), ans);
	}
}

impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let arr_len = arr_len.min(steps + 1) as usize;
        let steps = steps as usize + 1;
        let mut dp = vec![vec![0i64; arr_len]; steps];
        dp[0][0] = 1;
        for i in 1..steps {
            for j in 0..arr_len {
                dp[i][j] = dp[i-1][j];
                if j > 0 {
                    dp[i][j] += dp[i-1][j-1];
                }
                if j + 1 < arr_len {
                    dp[i][j] += dp[i-1][j+1];
                }
                dp[i][j] %= 10i64.pow(9) + 7;
            }
        }
        dp.last().unwrap()[0] as i32
    }
}