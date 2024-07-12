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
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut x = vec![vec![0; m + 1]; n + 1];
        let mut y = vec![vec![0; m + 1]; n + 1];
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                let a = if grid[i][j] == 'X' { 1 } else { 0 };
                let b = if grid[i][j] == 'Y' { 1 } else { 0 };
                x[i + 1][j + 1] = x[i][j + 1] + x[i + 1][j] - x[i][j] + a;
                y[i + 1][j + 1] = y[i][j + 1] + y[i + 1][j] - y[i][j] + b;
                if x[i + 1][j + 1] == y[i + 1][j + 1] && x[i + 1][j + 1] > 0 {
                    ans += 1;
                }
            }
        }
        ans
    }
}
