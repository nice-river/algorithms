#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let grid = to_vec2d([[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]]);
        let k = 1;
        let ans = 6;
        assert_eq!(Solution::shortest_path(grid, k), ans);
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

struct Solution {}

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut set = HashSet::new();
        let mut que = VecDeque::new();
        let n = grid.len();
        let m = grid[0].len();
        que.push_back((0i32, 0i32, 0, k));
        set.insert((0, 0, k));
        while let Some((x, y, step, k)) = que.pop_front() {
            if x as usize + 1 == n && y as usize + 1 == m {
                return step;
            }
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || ny < 0 || nx as usize >= n || ny as usize >= m {
                    continue;
                }
                if grid[nx as usize][ny as usize] == 0 {
                    if !set.contains(&(nx, ny, k)) {
                        que.push_back((nx, ny, step + 1, k));
                        set.insert((nx, ny, k));
                    }
                } else {
                    if k >= 1 && !set.contains(&(nx, ny, k - 1)) {
                        que.push_back((nx, ny, step + 1, k - 1));
                        set.insert((nx, ny, k - 1));
                    }
                }
            }
        }
        -1
    }
}
