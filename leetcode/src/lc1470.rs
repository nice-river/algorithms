#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![0, 1, 2, 3, 4, 5];
        let ans = vec![0, 3, 1, 4, 2, 5];
        assert_eq!(Solution::shuffle(nums, ans.len() as i32 / 2), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0; n * 2];
        for i in 0..n {
            ans[i * 2] = nums[i];
            ans[i * 2 + 1] = nums[i + n];
        }
        ans
    }
}
