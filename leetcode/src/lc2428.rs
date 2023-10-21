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

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut s = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                s[i + 1][j + 1] = s[i + 1][j] + grid[i][j];
            }
        }
        for i in 0..n {
            for j in 0..m {
                s[i + 1][j + 1] += s[i][j + 1];
            }
        }
        let mut ans = 0;
        for i in 2..n {
            for j in 2..m {
                let mut k = s[i + 1][j + 1] - s[i - 2][j + 1] - s[i + 1][j - 2] + s[i - 2][j - 2];
                k -= grid[i - 1][j];
                k -= grid[i - 1][j - 2];
                ans = ans.max(k);
            }
        }
        ans
    }
}
