#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let grid = vec![vec![1]];
        let row = 0;
        let col = 0;
        let color = 2;
        let ans = vec![vec![2]];
        assert_eq!(Solution::color_border(grid, row, col, color), ans);
    }
}

struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans = grid.clone();
        let mut vis = vec![vec![false; m]; n];
        let mut que = VecDeque::new();
        que.push_back((row, col));
        vis[row as usize][col as usize] = true;
        let dirs = vec![-1, 0, 1, 0, -1];
        while let Some((x, y)) = que.pop_front() {
            let c = grid[x as usize][y as usize];
            let mut k = 0;
            for d in 0..4 {
                let nx = x + dirs[d];
                let ny = y + dirs[d + 1];
                if nx < 0 || ny < 0 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if nx < n && ny < m && grid[nx][ny] == c {
                    k += 1;
                    if !vis[nx][ny] {
                        que.push_back((nx as i32, ny as i32));
                        vis[nx][ny] = true;
                    }
                }
            }
            if k < 4 {
                ans[x as usize][y as usize] = color;
            }
        }
        ans
    }
}
