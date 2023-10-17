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

use std::collections::BinaryHeap;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        const MODULE: i64 = 1_000_000_007;
        let mut heap = BinaryHeap::new();
        for num in nums {
            heap.push(-num);
        }
        for _ in 0..k {
            let t = heap.pop().unwrap();
            heap.push(t - 1);
        }
        let mut ans = 1;
        while let Some(t) = heap.pop() {
            ans = ans * (-t as i64);
            ans %= MODULE;
        }
        ans as i32
    }
}
