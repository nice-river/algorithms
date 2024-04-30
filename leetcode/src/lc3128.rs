#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let grid = to_vec2d([[0, 1, 0], [0, 1, 1], [0, 1, 0]]);
        let ans = 2;
        assert_eq!(Solution::number_of_right_triangles(grid), ans);
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
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();
        let m = grid[0].len();
        let mut rows = vec![0; n];
        let mut cols = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    ans += (rows[i] - 1) * (cols[j] - 1) as i64;
                }
            }
        }
        ans
    }
}
