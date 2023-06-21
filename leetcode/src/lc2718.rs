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

use std::collections::HashSet;

impl Solution {
    pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
        let mut row_set = HashSet::new();
        let mut col_set = HashSet::new();
        let n = n as i64;
        let mut ans = 0;
        for query in queries.into_iter().rev() {
            let tp = query[0];
            let idx = query[1];
            let val = query[2] as i64;
            if tp == 0 {
                if !row_set.contains(&idx) {
                    ans += val * (n - col_set.len() as i64);
                    row_set.insert(idx);
                }
            } else {
                if !col_set.contains(&idx) {
                    ans += val * (n - row_set.len() as i64);
                    col_set.insert(idx);
                }
            }
        }
        ans
    }
}
