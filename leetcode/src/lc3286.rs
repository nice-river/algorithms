#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        const DIRS: [i32; 5] = [-1, 0, 1, 0, -1];
        let mut pq = BinaryHeap::new();
        let n = grid.len();
        let m = grid[0].len();
        let mut vis = vec![vec![false; m]; n];
        pq.push(Reverse((grid[0][0], 0, 0)));
        vis[0][0] = true;
        while let Some(Reverse((h, x, y))) = pq.pop() {
            if x == n - 1 && y == m - 1 {
                return health > h;
            }
            for d in 0..4 {
                let nx = x as i32 + DIRS[d];
                let ny = y as i32 + DIRS[d + 1];
                if nx < 0 || ny < 0 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if nx >= n || ny >= m || vis[nx][ny] {
                    continue;
                }
                vis[nx][ny] = true;
                pq.push(Reverse((h + grid[nx][ny], nx, ny)));
            }
        }
        unreachable!()
    }
}
