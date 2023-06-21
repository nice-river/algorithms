#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mat = to_vec2d([[2, 5, 6], [3, 4, 4]]);
        let ans = 5;
        assert_eq!(Solution::max_increasing_cells(mat), ans);
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

use std::collections::BTreeMap;

impl Solution {
    pub fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut dp = vec![vec![0; m]; n];
        let mut row_maps: Vec<BTreeMap<i32, i32>> = vec![BTreeMap::new(); n];
        let mut col_maps: Vec<BTreeMap<i32, i32>> = vec![BTreeMap::new(); m];
        let mut vals = vec![];
        for i in 0..n {
            for j in 0..m {
                vals.push((mat[i][j], i, j));
            }
        }
        vals.sort_unstable();
        for (val, i, j) in vals.into_iter().rev() {
            let &a = row_maps[i].range(val + 1..).next().unwrap_or((&0, &0)).1;
            let &b = col_maps[j].range(val + 1..).next().unwrap_or((&0, &0)).1;
            let c = a.max(b) + 1;
            let e = row_maps[i].entry(val).or_insert(c);
            *e = (*e).max(c);
            let e = col_maps[j].entry(val).or_insert(c);
            *e = (*e).max(c);
            dp[i][j] = c;
        }

        *dp.iter().map(|x| x.iter().max().unwrap()).max().unwrap()
    }
}
