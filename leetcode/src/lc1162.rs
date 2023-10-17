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

struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut que = VecDeque::new();
        let n = grid.len();
        let mut vis = vec![vec![false; n]; n];
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    que.push_back((i, j, 0));
                    vis[i][j] = true;
                }
            }
        }
        while let Some((i, j, s)) = que.pop_front() {
            ans = s;
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = i as i32 + dx;
                let ny = j as i32 + dy;
                if nx < 0 || ny < 0 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if nx >= n || ny >= n || vis[nx][ny] {
                    continue;
                }
                que.push_back((nx, ny, s + 1));
                vis[nx][ny] = true;
            }
        }

        if ans == 0 {
            -1
        } else {
            ans
        }
    }
}
