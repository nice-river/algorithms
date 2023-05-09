#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 1, 1, 2, 2, 2, 3, 3, 3];
        let ans = 27;
        assert_eq!(Solution::find_number_of_lis(nums), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut longest = 1;
        let mut dp = vec![(0, 0); nums.len()];
        dp[0] = (1, 1);
        for i in 1..nums.len() {
            let mut cur_cnt = 1;
            let mut cur_len = 1;
            for j in 0..i {
                if nums[j] < nums[i] {
                    if cur_len < dp[j].1 + 1 {
                        cur_len = dp[j].1 + 1;
                        cur_cnt = dp[j].0;
                    } else if cur_len == dp[j].1 + 1 {
                        cur_cnt += dp[j].0;
                    }
                }
            }
            dp[i] = (cur_cnt, cur_len);
            longest = longest.max(cur_len);
        }
        for (cnt, len) in dp {
            if len == longest {
                ans += cnt;
            }
        }
        ans
    }
}
