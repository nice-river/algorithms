#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 2, 3];
        let k = 2;
        let ans = 20;
        assert_eq!(Solution::min_max_subarray_sum(nums, k), ans);

        let nums = vec![1, -3, 1];
        let k = 2;
        let ans = -6;
        assert_eq!(Solution::min_max_subarray_sum(nums, k), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![2, 2, 2];
        let k = 2;
        let ans = 20;
        assert_eq!(Solution::min_max_subarray_sum(nums, k), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn min_max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut nxt_mini = vec![0; n + 2];
        let mut pre_mini = vec![0; n + 2];
        let mut nxt_maxi = vec![0; n + 2];
        let mut pre_maxi = vec![0; n + 2];
        let mut stk1 = vec![(i64::MIN, 0)];
        let mut stk2 = vec![(i64::MAX, 0)];
        for i in 1..=n {
            let num = nums[i - 1] as i64;
            while num < stk1.last().unwrap().0 {
                stk1.pop();
            }
            pre_mini[i] = stk1.last().unwrap().1;
            stk1.push((num, i));
            while num >= stk2.last().unwrap().0 {
                stk2.pop();
            }
            pre_maxi[i] = stk2.last().unwrap().1;
            stk2.push((num, i));
        }
        stk1 = vec![(i64::MIN, n + 1)];
        stk2 = vec![(i64::MAX, n + 1)];
        for i in (1..=n).rev() {
            let num = nums[i - 1] as i64;
            while num <= stk1.last().unwrap().0 {
                stk1.pop();
            }
            nxt_mini[i] = stk1.last().unwrap().1;
            stk1.push((num, i));
            while num > stk2.last().unwrap().0 {
                stk2.pop();
            }
            nxt_maxi[i] = stk2.last().unwrap().1;
            stk2.push((num, i));
        }
        let mut ans = 0;
        for i in 1..=n {
            let a = Self::helper(i - pre_mini[i] - 1, nxt_mini[i] - i, k);
            let b = Self::helper(i - pre_maxi[i] - 1, nxt_maxi[i] - i, k);
            ans += nums[i - 1] as i64 * (a + b);
        }
        ans
    }

    fn helper(a: usize, b: usize, k: i32) -> i64 {
        let a = a as i64;
        let b = b as i64;
        let k = k as i64;
        let a = a.min(k - 1);
        let b = b.min(k);

        let n = (a + b - k).max(0);
        let mut ret = ((k - a) + (k - a + n - 1)) * n / 2;
        ret += (a - n + 1) * b;
        ret
    }
}