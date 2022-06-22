#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![0, 0, 1, 0];
        let ans = vec![2, 4];
        assert_eq!(Solution::max_score_indices(nums), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut zeros = vec![0; n + 2];
        let mut ones = vec![0; n + 2];
        for i in 1..=n {
            zeros[i] = zeros[i - 1] + if nums[i - 1] == 0 { 1 } else { 0 };
            ones[n - i + 1] = ones[n - i + 2] + if nums[n - i] == 1 { 1 } else { 0 };
        }
        let mut ans = vec![];
        let mut c = 0;

        for i in 1..=n + 1 {
            if c < zeros[i - 1] + ones[i] {
                ans = vec![i as i32 - 1];
                c = zeros[i - 1] + ones[i];
            } else if c == zeros[i - 1] + ones[i] {
                ans.push(i as i32 - 1);
            }
        }

        ans
    }
}
