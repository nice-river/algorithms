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
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n = mat.len();
        let m = mat[0].len();
        let mut cum = vec![vec![0; m + 1]; n];
        let k = k as usize;
        for i in 0..n {
            for j in 0..m {
                cum[i][j + 1] = cum[i][j] + mat[i][j];
            }
        }
        let mut ans = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                let a = i.checked_sub(k).unwrap_or(0);
                let b = (i + k).min(n - 1);
                let mut t = 0;
                let c = j.checked_sub(k).unwrap_or(0);
                let d = (j + k).min(m - 1);
                for r in a..=b {
                    t += cum[r][d + 1] - cum[r][c];
                }
                ans[i][j] = t;
            }
        }
        ans
    }
}
