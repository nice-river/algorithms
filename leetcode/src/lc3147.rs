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

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut ans = i32::MIN;
        let mut dp = vec![i32::MIN; energy.len()];
        for (i, &e) in energy.iter().enumerate() {
            dp[i] = e;
            if i >= k {
                dp[i] = (dp[i - k] + e).max(dp[i]);
            }
            if i + k >= energy.len() {
                ans = ans.max(dp[i]);
            }
        }
        ans
    }
}
