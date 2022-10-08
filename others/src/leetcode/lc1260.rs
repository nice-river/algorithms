#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test0() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let k = 8;
        let expected = vec![vec![4, 1], vec![2, 3]];
        let expected = grid.clone();
        assert_eq!(Solution::shift_grid(grid, k), expected);
    }
}

struct Solution {}

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n = grid.len();
        if n == 0 {
            return vec![];
        }
        let m = grid[0].len();
        let t = n * m;
        let k = k as usize % t;
        let mut ans = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                let g = ((i * m) + j + k) % t;
                ans[g / m][g % m] = grid[i][j];
            }
        }
        ans
    }
}
