#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let strs = vec!["abcdd".to_owned()];
        let ans = 0;
        assert_eq!(Solution::min_deletion_size(strs), ans);
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
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strs = strs.into_iter().map(|s| s.into_bytes()).collect::<Vec<_>>();
        let mut arr = vec![vec![0; strs.len()]; strs[0].len()];
        for (i, s) in strs.into_iter().enumerate() {
            for (j, b) in s.into_iter().enumerate() {
                arr[j][i] = b;
            }
        }
        let mut dp = vec![1; arr.len()];
        let mut longest = 1;
        for i in 1..arr.len() {
            for j in 0..i {
                if arr[i].iter().zip(arr[j].iter()).all(|(a, b)| a >= b) {
                    dp[i] = dp[i].max(dp[j] + 1);
                    longest = longest.max(dp[i]);
                }
            }
        }
        arr.len() as i32 - longest
    }
}
