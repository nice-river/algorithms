#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let grid = to_vec2d([
            [10, 1, 4, 8],
            [6, 6, 3, 10],
            [7, 4, 7, 10],
            [1, 10, 6, 1],
            [2, 1, 1, 10],
            [3, 8, 9, 2],
            [7, 1, 10, 10],
            [7, 1, 4, 9],
            [2, 2, 4, 2],
            [10, 7, 5, 10],
        ]);
        let k = 1;
        dbg!(Solution::rotate_grid(grid, k));
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

impl Solution {
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans = vec![vec![0; m]; n];
        let k = k as usize;
        for i in 0..n {
            let mut arr = vec![];
            for j in i + 1..m - i {
                arr.push(grid[i][j]);
            }
            for j in i + 1..n - i {
                arr.push(grid[j][m - 1 - i]);
            }
            for j in (i..m - 1 - i).rev() {
                arr.push(grid[n - 1 - i][j]);
            }
            for j in (i..n - 1 - i).rev() {
                arr.push(grid[j][i]);
            }
            let mut p = 0;
            for j in i + 1..m - i {
                ans[i][j] = arr[(p + k) % arr.len()];
                p += 1;
            }
            for j in i + 1..n - i {
                ans[j][m - 1 - i] = arr[(p + k) % arr.len()];
                p += 1;
            }
            for j in (i..m - 1 - i).rev() {
                ans[n - 1 - i][j] = arr[(p + k) % arr.len()];
                p += 1;
            }
            for j in (i..n - 1 - i).rev() {
                ans[j][i] = arr[(p + k) % arr.len()];
                p += 1;
            }
            if i + 1 == n - 1 - i || i + 1 == m - 1 - i {
                break;
            }
        }
        ans
    }
}
