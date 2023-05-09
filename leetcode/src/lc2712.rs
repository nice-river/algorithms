#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let num_slots = 3;
        let ans = 9;
        assert_eq!(Solution::maximum_and_sum(nums, num_slots), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![2, 3, 4, 6];
        let num_slots = 3;
        let ans = 7;
        assert_eq!(Solution::maximum_and_sum(nums, num_slots), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![8, 10, 10, 7, 5, 7, 10, 1, 2, 2, 5, 8];
        let num_slots = 7;
        let ans = 35;
        assert_eq!(Solution::maximum_and_sum(nums, num_slots), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![8, 13, 3, 15, 3, 15, 2, 15, 5, 7, 6];
        let num_slots = 8;
        let ans = 60;
        assert_eq!(Solution::maximum_and_sum(nums, num_slots), ans);
    }

    #[test]
    fn test4() {
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let num_slots = 9;
        let ans = 10;
        assert_eq!(Solution::maximum_and_sum(nums, num_slots), ans);
    }

    #[test]
    fn test5() {
        let nums = vec![1, 1];
        let num_slots = 1;
        let ans = 2;
        assert_eq!(Solution::maximum_and_sum(nums, num_slots), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn maximum_and_sum(mut nums: Vec<i32>, num_slots: i32) -> i32 {
        let num_slots = num_slots as usize * 2;
        let mut dp = vec![0; 1 << num_slots];
        let mut ans = 0;
        for i in 0..dp.len() {
            ans = ans.max(dp[i]);
            let c = i.count_ones() as usize;
            if c >= nums.len() {
                continue;
            }
            for j in 0..num_slots {
                if ((1 << j) & i) == 0 {
                    dp[(1 << j) ^ i] = dp[(1 << j) ^ i].max(dp[i] + (nums[c] & (j as i32 / 2) + 1));
                }
            }
        }
        ans
    }
}
