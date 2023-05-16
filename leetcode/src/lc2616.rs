#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![4, 2, 1, 2];
        let p = 2;
        let ans = 2;
        assert_eq!(Solution::minimize_max(nums, p), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![1, 2, 10, 20, 21];
        let p = 2;
        let ans = 1;
        assert_eq!(Solution::minimize_max(nums, p), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }
        nums.sort_unstable();
        let mut diff = vec![0; nums.len() - 1];
        for i in 1..nums.len() {
            diff[i - 1] = nums[i] - nums[i - 1];
        }
        let mut l = 0;
        let mut r = nums[nums.len() - 1] - nums[0];
        let mut ans = r;
        while l <= r {
            let m = l + (r - l) / 2;
            if Self::check(&diff, m, p) {
                ans = m;
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        ans
    }

    fn check(diff: &Vec<i32>, k: i32, p: i32) -> bool {
        let mut dp = vec![0; 2];
        if diff[0] <= k {
            dp[0] = 1;
        }
        if dp[0] >= p || dp[1] >= p {
            return true;
        }
        for i in 1..diff.len() {
            let mut tmp = vec![0, dp[0].max(dp[1])];
            if diff[i] <= k {
                tmp[0] = dp[1] + 1;
            } else {
                tmp[0] = tmp[1];
            }
            dp = tmp;
            if dp[0] >= p || dp[1] >= p {
                return true;
            }
        }
        false
    }
}
