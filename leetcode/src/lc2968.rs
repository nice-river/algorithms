#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 2, 6, 4];
        let k = 3;
        let ans = 3;
        assert_eq!(Solution::max_frequency_score(nums, k), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![10, 19, 26, 18, 27, 18];
        let k = 9;
        let ans = 4;
        assert_eq!(Solution::max_frequency_score(nums, k), ans);
    }

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_frequency_score(nums: Vec<i32>, k: i64) -> i32 {
        let mut nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        nums.sort();
        let n = nums.len();
        let mut s = vec![0; n + 1];
        for i in 0..nums.len() {
            s[i + 1] = s[i] + nums[i];
        }
        let mut ans = 1;
        let mut l = 2;
        let mut r = n;
        while l <= r {
            let m = (l + r) / 2;
            let mut checked = false;

            for i in 0..n - m + 1 {
                let a = (i + i + m - 1) / 2;
                let x = nums[a];
                let t =
                    x * (a - i) as i64 - (s[a] - s[i]) + (s[i + m] - s[a]) - x * (i + m - a) as i64;
                if t <= k {
                    checked = true;
                    break;
                }
                let a = (i + i + m - 1) / 2 + 1;
                let x = nums[a];
                let t =
                    x * (a - i) as i64 - (s[a] - s[i]) + (s[i + m] - s[a]) - x * (i + m - a) as i64;
                if t <= k {
                    checked = true;
                    break;
                }
            }
            if checked {
                l = m + 1;
                ans = m;
            } else {
                r = m - 1;
            }
        }

        ans as i32
    }
}
