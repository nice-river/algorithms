struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let nums1 = vec![1,1,3,5,3,3,5,5,1,1];
        let nums2 = vec![2,3,2,1,3,5,3,2,2,1];
        assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), 5);
	}
}

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums1.len() + 1]; 2];
        let mut cur = 0;
        for num in nums2 {
            let nxt = cur ^ 1;
            for j in 1..=nums1.len() {
                dp[nxt][j] = dp[nxt][j-1].max(dp[cur][j]);
                if num == nums1[j-1] {
                    dp[nxt][j] = dp[nxt][j].max(dp[cur][j-1] + 1);
                }
            }
            cur ^= 1;
        }
        *dp[cur].last().unwrap()
    }
}