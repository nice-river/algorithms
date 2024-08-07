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
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        let mut tmp = 0;
        for i in 0..n {
            for j in 0..m / 2 {
                if grid[i][j] != grid[i][m - 1 -j] {
                    tmp += 1;
                }
            }
        }
        let ans = tmp;
        tmp = 0;
        for j in 0..m {
            for i in 0..n / 2 {
                if grid[i][j] != grid[n - 1 - i][j] {
                    tmp += 1;
                }
            }
        }
        ans.min(tmp)
    }
}
