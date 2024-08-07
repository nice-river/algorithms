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

use crate::*;

use std::collections::HashMap;

struct neighborSum {
    grid: Vec<Vec<i32>>,
    num_to_axis: HashMap<i32, (i32, i32)>,
    n: i32,
    m: i32,
}

impl neighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut num_to_axis = HashMap::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                num_to_axis.insert(grid[i][j], (i as i32, j as i32));
            }
        }
        let n = grid.len() as i32;
        let m = grid[0].len() as i32;

        Self {
            grid,
            num_to_axis,
            n,
            m,
        }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        let mut ret = 0;
        if let Some(&(x, y)) = self.num_to_axis.get(&value) {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let x = x + dx;
                let y = y + dy;
                if x < 0 || y < 0 || x >= self.n || y >= self.m {
                    continue;
                }
                ret += self.grid[x as usize][y as usize];
            }
        }
        ret
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        let mut ret = 0;
        if let Some(&(x, y)) = self.num_to_axis.get(&value) {
            for (dx, dy) in [(-1, -1), (1, 1), (1, -1), (-1, 1)] {
                let x = x + dx;
                let y = y + dy;
                if x < 0 || y < 0 || x >= self.n || y >= self.m {
                    continue;
                }
                ret += self.grid[x as usize][y as usize];
            }
        }
        ret
    }
}
