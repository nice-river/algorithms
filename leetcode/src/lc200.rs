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
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '1' {
                    ans += 1;
                    Self::mark(&mut grid, i, j);
                }
            }
        }
        ans
    }

    fn mark(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let n = grid.len();
        let m = grid[0].len();
        grid[i][j] = '0';
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let ni = i as i32 + dx;
            let nj = j as i32 + dy;
            if ni < 0 || nj < 0 || ni as usize >= n || nj as usize >= m {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if grid[ni][nj] == '1' {
                Self::mark(grid, ni, nj);
            }
        }
    }
}
