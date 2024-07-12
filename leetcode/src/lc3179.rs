#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 4;
        let k = 1;
        let ans = 4;
        assert_eq!(Solution::value_after_k_seconds(n, k), ans);
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
    pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
        const MODULE: i64 = 10i64.pow(9) + 7;
        let mut arr = vec![1; n as usize];
        for _ in 0..k {
            let mut s = 0;
            (0..n as usize).for_each(|i| {
                arr[i] += s;
                arr[i] %= MODULE;
                s = arr[i];
            });
        }
        *arr.last().unwrap() as i32
    }
}
