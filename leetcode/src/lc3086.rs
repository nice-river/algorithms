#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1];
        let k = 3;
        let max_changes = 1;
        let ans = 3;
        assert_eq!(Solution::minimum_moves(nums, k, max_changes), ans);

        // let nums = vec![0, 0, 0, 0];
        // let k = 2;
        // let max_changes = 3;
        // let ans = 4;
        // assert_eq!(Solution::minimum_moves(nums, k, max_changes), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![0, 1];
        let k = 2;
        let max_changes = 5;
        let ans = 2;
        assert_eq!(Solution::minimum_moves(nums, k, max_changes), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 0, 1, 0, 1];
        let k = 3;
        let max_changes = 0;
        let ans = 4;
        assert_eq!(Solution::minimum_moves(nums, k, max_changes), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 1];
        let k = 1;
        let max_changes = 2;
        let ans = 0;
        assert_eq!(Solution::minimum_moves(nums, k, max_changes), ans);
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
    pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
        let mut arr = vec![0; nums.len()];
        let mut next_one = vec![nums.len(); nums.len()];
        for i in (0..nums.len() - 1).rev() {
            next_one[i] = next_one[i + 1];
            if nums[i + 1] == 1 {
                next_one[i] = i + 1;
            }
        }
        if k > max_changes {
            let mut n = k - max_changes;
            let mut a = if nums[0] == 1 { 0 } else { next_one[0] };
            n -= 1;
            let mut b = a;
            let mut x = 1;
            let mut y = 0;
            let mut t = 0;
            while n > 0 {
                b = next_one[b];
                y += 1;
                t += (b - a) as i64;
                n -= 1;
            }
            let mut i = a;
            while i != nums.len() {
                arr[i] = t;
                let j = next_one[i];
                if j == nums.len() {
                    break;
                }
                t += x * (j - i) as i64;
                t -= y * (j - i) as i64;
                x += 1;
                y -= 1;
                i = j;
                while b < nums.len()
                    && next_one[b] != nums.len()
                    && a != i
                    && next_one[b] - i < i - a
                {
                    t -= (i - a) as i64;
                    t += (next_one[b] - i) as i64;
                    a = next_one[a];
                    b = next_one[b];
                    x -= 1;
                    y += 1;
                }
            }
        }
        let mut i = if nums[0] == 1 { 0 } else { next_one[0] };
        if i == nums.len() {
            return k as i64 * 2;
        }
        // dbg!(&arr);
        let mut ans = i64::MAX;
        while i != nums.len() {
            let mut g = 1;
            if i + 1 != nums.len() && nums[i + 1] == 1 {
                g += 1;
            }
            if i > 0 && nums[i - 1] == 1 {
                g += 1;
            }
            if g >= k {
                ans = ans.min(k as i64 - 1);
            } else {
                if max_changes >= k - g {
                    ans = ans.min(g as i64 - 1 + (k - g) as i64 * 2);
                } else {
                    ans = ans.min(arr[i] + (2 * k - max_changes).min(max_changes) as i64 * 2);
                }
            }
            i = next_one[i];
        }
        ans
    }
}
