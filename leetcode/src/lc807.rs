#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut n = grid.len();
        let mut m = grid[0].len();
        let mut row_max = vec![0; n];
        let mut col_max = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                row_max[i] = row_max[i].max(grid[i][j]);
                col_max[j] = col_max[j].max(grid[i][j]);
            }
        }
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                ans += row_max[i].min(col_max[j]) - grid[i][j];
            }
        }
        ans
    }
}
