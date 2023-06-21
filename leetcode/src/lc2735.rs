#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![31, 25, 18, 59];
        let x = 27;
        let ans = 119;
        assert_eq!(Solution::min_cost(nums, x), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![4, 1, 5, 9];
        let x = 2;
        let ans = 10;
        assert_eq!(Solution::min_cost(nums, x), ans);
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

struct Solution {}

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
        let mut nums = nums.into_iter().map(|e| e as i64).collect::<Vec<_>>();
        let n = nums.len();
        nums.extend_from_within(..);
        let x = x as i64;
        let mut ans = i64::MAX;
        for round in 0..n {
            let mut res = round as i64 * x;
            let mut heap = BinaryHeap::new();
            for i in n..n + round {
                heap.push((Reverse(nums[i]), i));
            }
            for i in (0..n).rev() {
                heap.push((Reverse(nums[i]), i));
                while let Some((Reverse(val), p)) = heap.peek() {
                    if *p > i + round {
                        heap.pop();
                    } else {
                        res += val.min(&nums[i]);
                        break;
                    }
                }
            }

            ans = ans.min(res);
        }
        ans
    }
}
