#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let grid = to_vec2d([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
        let ans = 3;
        assert_eq!(Solution::min_flips(grid), ans);
    }

    #[test]
    fn test1() {
        let grid = to_vec2d([[0, 1, 0], [0, 1, 0], [0, 1, 1]]);
        let ans = 4;
        assert_eq!(Solution::min_flips(grid), ans);
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
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        let mut ans = 0;
        for i in 0..n / 2 {
            for j in 0..m / 2 {
                let t = grid[i][j]
                    + grid[n - 1 - i][j]
                    + grid[i][m - 1 - j]
                    + grid[n - 1 - i][m - 1 - j];
                ans += t.min(4 - t);
            }
        }
        if n % 2 == 1 && m % 2 == 1 && grid[n / 2][m / 2] == 1 {
            ans += 1;
        }
        let mut ones = 0;
        let mut diff = 0;
        if n % 2 == 1 {
            for i in 0..m / 2 {
                if grid[n / 2][i] == grid[n / 2][m - 1 - i] {
                    ones += if grid[n / 2][i] == 1 { 2 } else { 0 };
                } else {
                    diff += 1;
                }
            }
        }
        if m % 2 == 1 {
            for i in 0..n / 2 {
                if grid[i][m / 2] == grid[n - 1 - i][m / 2] {
                    ones += if grid[i][m / 2] == 1 { 2 } else { 0 };
                } else {
                    diff += 1;
                }
            }
        }
        if ones % 4 == 2 {
            ans += if diff > 0 { diff } else { 2 };
        } else {
            ans += diff;
        }
        ans
    }
}
