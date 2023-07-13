#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let m = 3;
        let n = 3;
        let coordinates = to_vec2d([[0, 0], [1, 1], [0, 2]]);
        let ans = vec![0, 2, 2, 0, 0];
        assert_eq!(Solution::count_black_blocks(m, n, coordinates), ans);
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

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_black_blocks(m: i32, n: i32, coordinates: Vec<Vec<i32>>) -> Vec<i64> {
        let mut coords = HashSet::new();
        for coord in coordinates {
            coords.insert((coord[0], coord[1]));
        }
        let mut ans = vec![0; 5];
        let mut map = HashMap::new();
        for coord in coords.iter() {
            for (di, dj) in [(-1, 0), (0, -1), (-1, -1), (0, 0)] {
                let i = coord.0 + di;
                let j = coord.1 + dj;
                if i < 0 || j < 0 || i == m - 1 || j == n - 1 {
                    continue;
                }
                if map.contains_key(&(i, j)) {
                    continue;
                }
                let mut t = 0;
                for (dx, dy) in [(1, 0), (0, 1), (1, 1), (0, 0)] {
                    let nx = i + dx;
                    let ny = j + dy;
                    if coords.contains(&(nx, ny)) {
                        t += 1;
                    }
                }
                map.insert((i, j), t);
            }
        }
        let all = (m as i64 - 1) * (n as i64 - 1);
        ans[0] = all;
        for (_, v) in map.into_iter() {
            ans[v] += 1;
            ans[0] -= 1;
        }
        ans
    }
}
