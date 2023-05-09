#[cfg(test)]
mod tests {
    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] > 0 {
                    ans = ans.max(Self::dfs(&mut grid, i, j));
                }
            }
        }
        ans
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
        let n = grid.len() as i32;
        let m = grid[0].len() as i32;

        let mut r = grid[x][y];
        grid[x][y] = 0;
        let dirs = [-1, 0, 1, 0, -1];
        let x = x as i32;
        let y = y as i32;
        for d in 0..4 {
            let nx = x as i32 + dirs[d];
            let ny = y as i32 + dirs[d + 1];
            if nx >= n || ny >= m || nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if grid[nx][ny] > 0 {
                r += Self::dfs(grid, nx, ny);
            }
        }
        r
    }
}
