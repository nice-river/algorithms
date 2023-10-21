#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let houses = vec![1, 4, 8, 10, 20];
        let k = 3;
        let ans = 5;
        assert_eq!(Solution::min_distance(houses, k), ans);
    }

    #[test]
    fn test1() {
        let houses = vec![1, 4, 5];
        let k = 2;
        let ans = 1;
        assert_eq!(Solution::min_distance(houses, k), ans);
    }

    #[test]
    fn test2() {
        let houses = vec![
            25, 35, 49, 31, 13, 3, 26, 39, 20, 12, 23, 32, 45, 34, 48, 21,
        ];
        let k = 6;
        let ans = 20;
        assert_eq!(Solution::min_distance(houses, k), ans);
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
    pub fn min_distance(mut houses: Vec<i32>, k: i32) -> i32 {
        houses.sort();
        let k = k as usize;
        let n = houses.len();
        let mut dp = vec![vec![i32::MAX; n]; k + 1];
        let mut cost0 = vec![vec![0; n]; n];
        let mut cost1 = vec![vec![0; n]; n];
        for i in 0..n {
            cost0[i][i] = 0;
            for j in i + 1..n {
                cost0[i][j] = cost0[i][j - 1] + houses[j] - houses[i];
            }
            cost1[i][i] = 0;
            for j in (0..i).rev() {
                cost1[j][i] = cost1[j + 1][i] + houses[i] - houses[j];
            }
        }
        for i in 0..n {
            let m = i / 2;
            dp[1][i] = cost1[0][m] + cost0[m][i];
        }
        for i in 2..=k {
            for j in 0..n {
                if j + 1 <= i {
                    dp[i][j] = 0;
                } else {
                    for c in 0..j {
                        if dp[i - 1][c] != i32::MAX {
                            let m = (c + 1 + j) / 2;
                            dp[i][j] = dp[i][j].min(dp[i - 1][c] + cost1[c + 1][m] + cost0[m][j]);
                        }
                    }
                }
            }
        }
        dp[k][n - 1]
    }
}
