#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 6;
        let lamps = to_vec2d([
            [2, 5],
            [4, 2],
            [0, 3],
            [0, 5],
            [1, 4],
            [4, 2],
            [3, 3],
            [1, 0],
        ]);
        let queries = to_vec2d([[4, 3], [3, 1], [5, 3], [0, 5], [4, 4], [3, 3]]);
        let ans = vec![1, 0, 1, 1, 0, 1];
        assert_eq!(Solution::grid_illumination(n, lamps, queries), ans);
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

struct Axis {
    x: usize,
    y: usize,
    n: usize,
}

impl Axis {
    fn new(n: usize) -> Self {
        Self { x: 0, y: 0, n }
    }

    fn update(&mut self, x: i32, y: i32) {
        self.x = x as usize;
        self.y = y as usize;
    }

    fn row(&self) -> usize {
        self.x
    }

    fn col(&self) -> usize {
        self.y
    }

    fn diag(&self) -> usize {
        (self.x as i64 - self.y as i64 + self.n as i64) as usize
    }

    fn anti_diag(&self) -> usize {
        self.x + self.y
    }
}

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut rows = HashMap::new();
        let mut cols = HashMap::new();
        let mut diags = HashMap::new();
        let mut anti_diags = HashMap::new();

        let mut ans = vec![];
        let mut ax = Axis::new(n);

        let mut on = HashSet::new();

        for lamp in lamps {
            let x = lamp[0];
            let y = lamp[1];
            if on.contains(&(x, y)) {
                continue;
            }
            on.insert((x, y));
            ax.update(x, y);
            *rows.entry(ax.row()).or_insert(0) += 1;
            *cols.entry(ax.col()).or_insert(0) += 1;
            *diags.entry(ax.diag()).or_insert(0) += 1;
            *anti_diags.entry(ax.anti_diag()).or_insert(0) += 1;
        }

        for query in queries {
            let x = query[0];
            let y = query[1];
            ax.update(x, y);
            if rows.get(&ax.row()).unwrap_or(&0) >= &1
                || cols.get(&ax.col()).unwrap_or(&0) >= &1
                || diags.get(&ax.diag()).unwrap_or(&0) >= &1
                || anti_diags.get(&ax.anti_diag()).unwrap_or(&0) >= &1
            {
                ans.push(1);
            } else {
                ans.push(0);
            }
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let i = x + dx;
                    let j = y + dy;
                    if i < 0 || j < 0 || i >= n as i32 || j >= n as i32 {
                        continue;
                    }
                    if !on.contains(&(i, j)) {
                        continue;
                    }
                    ax.update(i, j);
                    *rows.entry(ax.row()).or_insert(0) -= 1;
                    *cols.entry(ax.col()).or_insert(0) -= 1;
                    *diags.entry(ax.diag()).or_insert(0) -= 1;
                    *anti_diags.entry(ax.anti_diag()).or_insert(0) -= 1;
                    on.remove(&(i, j));
                }
            }
        }

        ans
    }
}
