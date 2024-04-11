#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let rods = vec![1, 2, 3];
        let ans = 3;
        assert_eq!(Solution::tallest_billboard(rods), ans);

        let rods = vec![1, 2];
        let ans = 0;
        assert_eq!(Solution::tallest_billboard(rods), ans);
    }

    #[test]
    fn test1() {
        let rods = vec![1, 2, 3, 4, 5, 6];
        let ans = 10;
        assert_eq!(Solution::tallest_billboard(rods), ans);
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
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        const N: usize = 5010;
        let mut dp = vec![-1; N * 2];
        dp[N] = 0;
        for rod in rods {
            let rod = rod as usize;
            let mut cur = dp.clone();
            for i in 0..dp.len() {
                if dp[i] != -1 {
                    if i + rod < N * 2 {
                        cur[i + rod] = cur[i + rod].max(dp[i] + rod as i32);
                    }
                    if i >= rod {
                        cur[i - rod] = cur[i - rod].max(dp[i]);
                    }
                }
            }
            dp = cur;
        }
        dp[N]
    }
}
