#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 2, 3, 4];
        let k = 3;
        let ans = 4;
        assert_eq!(Solution::sum_of_powers(nums, k), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![4, 3, -1];
        let k = 2;
        let ans = 10;
        assert_eq!(Solution::sum_of_powers(nums, k), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![-15, 5, 11, 17, -11, -23];
        let k = 6;
        let ans = 4;
        assert_eq!(Solution::sum_of_powers(nums, k), ans);
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

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn sum_of_powers(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let k = k as usize;
        nums.sort();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let diff = nums[j] - nums[i];
                let (forward, backward) = Self::construct(&nums, diff, k);
                let mut left = None;
                let mut right = None;
                for t in (0..i).rev() {
                    if nums[i] - nums[t] > diff {
                        left = Some(t);
                        break;
                    }
                }
                for t in j + 1..nums.len() {
                    if nums[t] - nums[j] >= diff {
                        right = Some(t);
                        break;
                    }
                }
                match (left, right) {
                    (Some(left), Some(right)) => {
                        for i in 0..=k - 2 {
                            ans += forward[left][i] * backward[right][k - 2 - i] % MOD
                                * diff as i64
                                % MOD;
                            ans %= MOD;
                        }
                    }
                    (Some(left), None) => {
                        ans += forward[left][k - 2] * diff as i64 % MOD;
                        ans %= MOD;
                    }
                    (None, Some(right)) => {
                        ans += backward[right][k - 2] * diff as i64 % MOD;
                        ans %= MOD;
                    }
                    (None, None) => {
                        if k == 2 {
                            ans += diff as i64;
                            ans %= MOD;
                        }
                    }
                }
            }
        }

        ans as i32
    }

    fn construct(nums: &Vec<i32>, diff: i32, k: usize) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
        let mut forward = vec![vec![0; k]; nums.len()];
        let mut backward = vec![vec![0; k]; nums.len()];
        for i in 0..nums.len() {
            for j in 0..=1 {
                forward[i][j] = 1;
                backward[i][j] = 1;
            }
        }
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] - nums[j] > diff {
                    for g in 2..k {
                        forward[i][g] += forward[j][g - 1];
                        forward[i][g] %= MOD;
                    }
                }
            }
        }
        for i in (0..nums.len() - 1).rev() {
            for j in i + 1..nums.len() {
                if nums[j] - nums[i] >= diff {
                    for g in 2..k {
                        backward[i][g] += backward[j][g - 1];
                        backward[i][g] %= MOD;
                    }
                }
            }
        }
        for i in 1..nums.len() {
            for j in 1..k {
                forward[i][j] += forward[i - 1][j];
                forward[i][j] %= MOD;
            }
        }
        for i in (0..nums.len() - 1).rev() {
            for j in 1..k {
                backward[i][j] += backward[i + 1][j];
                backward[i][j] %= MOD;
            }
        }

        (forward, backward)
    }
}
