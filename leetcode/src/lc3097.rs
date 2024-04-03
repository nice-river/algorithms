#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 2, 3];
        let k = 2;
        let ans = 1;
        assert_eq!(Solution::minimum_subarray_length(nums, k), ans);

        let nums = vec![2, 1, 8];
        let k = 10;
        let ans = 3;
        assert_eq!(Solution::minimum_subarray_length(nums, k), ans);

        let nums = vec![1, 2];
        let k = 0;
        let ans = 1;
        assert_eq!(Solution::minimum_subarray_length(nums, k), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![3, 1, 4, 2];
        let k = 7;
        let ans = 3;
        assert_eq!(Solution::minimum_subarray_length(nums, k), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 32, 21];
        let k = 55;
        let ans = 3;
        assert_eq!(Solution::minimum_subarray_length(nums, k), ans);
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

const N: usize = 32;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut bits = vec![0; N];
        let mut ans: Option<i32> = None;
        let mut left = 0;
        for i in 0..nums.len() {
            if nums[i] >= k {
                return 1;
            }
            let n = Self::add(&mut bits, nums[i]);
            if n >= k {
                let d = (i - left + 1) as i32;
                ans = Some(ans.map_or(d, |v| v.min(d)));
                while left < i && Self::try_sub(&bits, nums[left]) >= k {
                    let _ = Self::sub(&mut bits, nums[left]);
                    left += 1;
                    let d = (i - left + 1) as i32;
                    ans = Some(ans.map_or(d, |v| v.min(d)));
                }
            }
        }
        ans.unwrap_or(-1)
    }

    fn or_num(bits: &Vec<i32>) -> i32 {
        let mut n = 0;
        for i in 0..N {
            if bits[i] > 0 {
                n |= 1 << i;
            }
        }
        n
    }

    fn add(bits: &mut Vec<i32>, mut n: i32) -> i32 {
        let mut i = 0;
        while n != 0 {
            bits[i] += n % 2;
            n >>= 1;
            i += 1;
        }
        Self::or_num(bits)
    }

    fn sub(bits: &mut Vec<i32>, mut n: i32) -> i32 {
        let mut i = 0;
        while n != 0 {
            bits[i] -= n % 2;
            n >>= 1;
            i += 1;
        }
        Self::or_num(bits)
    }

    fn try_sub(bits: &Vec<i32>, mut n: i32) -> i32 {
        let mut r = 0;
        for i in 0..N {
            if bits[i] - n % 2 > 0 {
                r |= 1 << i;
            }
            n >>= 1;
        }
        r
    }
}
