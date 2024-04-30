#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let zero = 1;
        let one = 1;
        let limit = 2;
        let ans = 2;
        assert_eq!(Solution::number_of_stable_arrays(zero, one, limit), ans);
    }

    #[test]
    fn test1() {
        let zero = 1;
        let one = 2;
        let limit = 1;
        let ans = 1;
        assert_eq!(Solution::number_of_stable_arrays(zero, one, limit), ans);
    }

    #[test]
    fn test2() {
        let zero = 3;
        let one = 3;
        let limit = 2;
        let ans = 14;
        assert_eq!(Solution::number_of_stable_arrays(zero, one, limit), ans);
    }

    #[test]
    fn test3() {
        let zero = 0;
        let one = 2;
        let limit = 1;
        let ans = 0;
        assert_eq!(Solution::number_of_stable_arrays(zero, one, limit), ans);
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

const MODULE: i64 = 1_000_000_007;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let zero = zero as usize;
        let one = one as usize;
        let limit = limit as usize;

        let mut dp = vec![vec![vec![-1; 2]; one + 1]; zero + 1];

        let ans = (Self::dfs([zero, one], 0, limit, &mut dp)
            + Self::dfs([zero, one], 1, limit, &mut dp))
            % MODULE;
        ans as i32
    }

    fn dfs(
        mut bits: [usize; 2],
        last_bit: usize,
        limit: usize,
        dp: &mut Vec<Vec<Vec<i64>>>,
    ) -> i64 {
        let zero = bits[0];
        let one = bits[1];
        if dp[zero][one][last_bit] != -1 {
            return dp[zero][one][last_bit];
        }
        if bits[last_bit] == 0 && bits[last_bit ^ 1] != 0 {
            return 0;
        }
        if zero + one <= 1 {
            return 1;
        }
        let mut ret = 0;
        bits[last_bit] -= 1;
        ret += Self::dfs(bits, last_bit ^ 1, limit, dp);
        ret += Self::dfs(bits, last_bit, limit, dp);
        ret %= MODULE;
        let n = zero + one;
        if bits[last_bit] >= limit {
            bits[last_bit] -= limit;
            ret -= Self::dfs(bits, last_bit ^ 1, limit, dp);
            ret += MODULE;
            ret %= MODULE;
        }

        dp[zero][one][last_bit] = ret;
        ret
    }
}
