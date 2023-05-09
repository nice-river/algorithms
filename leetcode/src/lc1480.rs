struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2];
        let ans = vec![1, 3];
        assert_eq!(Solution::running_sum(nums), ans);
    }
}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        ans[0] = nums[0];
        for i in 1..ans.len() {
            ans[i] = ans[i - 1] + nums[i];
        }
        ans
    }
}
