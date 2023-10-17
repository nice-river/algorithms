#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

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

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let n = nums.len();
        heap.push((Reverse(0), 0));
        for (i, num) in nums.into_iter().enumerate() {
            while let Some((Reverse(t), p)) = heap.pop() {
                if p >= i {
                    heap.push((Reverse(t), p));
                    heap.push((Reverse(t + 1), i + num as usize));
                    break;
                }
            }
        }
        while let Some((Reverse(t), p)) = heap.pop() {
            if p >= n - 1 {
                return t;
            }
        }
        unreachable!()
    }
}
