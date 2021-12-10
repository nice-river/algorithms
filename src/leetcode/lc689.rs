#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 1, 2, 2, 3, 3];
        let k = 2;
        let ans = vec![0, 2, 4];
        assert_eq!(Solution::max_sum_of_three_subarrays(nums, k), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut su = 0;
        for i in 0..k {
            su += nums[i];
        }
        let mut dp = vec![vec![(0, vec![]); 4]; nums.len()];
        dp[k - 1][1] = (su, vec![0]);
        for i in k..nums.len() {
            su += nums[i];
            su -= nums[i - k];
            for j in 1..=3 {
                dp[i][j] = dp[i - 1][j].clone();
                if dp[i - k][j - 1].0 + su > dp[i][j].0 {
                    dp[i][j].0 = dp[i - k][j - 1].0 + su;
                    dp[i][j].1 = dp[i - k][j - 1].1.clone();
                    dp[i][j].1.push(i - k + 1);
                }
            }
        }
        dp[nums.len() - 1][3].1.iter().map(|&x| x as i32).collect()
    }
}
