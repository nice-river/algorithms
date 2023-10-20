#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let locations = vec![2, 3, 6, 8, 4];
        let start = 1;
        let finish = 3;
        let fuel = 5;
        let ans = 4;
        assert_eq!(Solution::count_routes(locations, start, finish, fuel), ans);
    }

    #[test]
    fn test1() {
        let locations = vec![4, 3, 1];
        let start = 1;
        let finish = 0;
        let fuel = 6;
        let ans = 5;
        assert_eq!(Solution::count_routes(locations, start, finish, fuel), ans);
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
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        const MODULE: i64 = 1_000_000_007;
        let start = start as usize;
        let finish = finish as usize;
        let fuel = fuel as usize;
        let mut dp = vec![vec![0; locations.len()]; fuel + 1];
        dp[fuel][start] = 1;
        for f in (0..=fuel).rev() {
            for i in 0..locations.len() {
                if dp[f][i] != 0 {
                    for j in 0..locations.len() {
                        let c = (locations[i] - locations[j]).abs() as usize;
                        if i != j && c <= f {
                            dp[f - c][j] += dp[f][i];
                            dp[f - c][j] %= MODULE;
                        }
                    }
                }
            }
        }

        let mut ans = 0;
        for i in 0..=fuel as usize {
            ans += dp[i][finish];
            ans %= MODULE;
        }
        ans as i32
    }
}
