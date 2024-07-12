#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let power = vec![7, 1, 6, 6];
        let ans = 13;
        assert_eq!(Solution::maximum_total_damage(power), ans);
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
    pub fn maximum_total_damage(mut power: Vec<i32>) -> i64 {
        power.sort();
        let mut dp = vec![];
        let mut vals = vec![];
        for p in power {
            if vals.is_empty() || *vals.last().unwrap() != p {
                vals.push(p);
                dp.push(p as i64);
            } else {
                *dp.last_mut().unwrap() += p as i64;
            }
        }
        for i in 1..vals.len() {
            let mut v = 0;
            for j in (0..=i - 1).rev() {
                if vals[i] - vals[j] > 2 {
                    dp[i] += dp[j];
                    break;
                }
                v = v.max(dp[j]);
            }
            dp[i] = dp[i].max(v);
        }
        *dp.last().unwrap()
    }
}
