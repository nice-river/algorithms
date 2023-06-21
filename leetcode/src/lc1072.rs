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

use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut set: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut ans = 0;
        for row in matrix {
            let mut flip_row = row.clone();
            flip_row.iter_mut().for_each(|x| *x ^= 1);
            let a = *set.get(&row).unwrap_or(&0);
            let b = *set.get(&flip_row).unwrap_or(&0);
            ans = ans.max(a + b + 1);
            *set.entry(row).or_insert(0) += 1;
        }
        ans
    }
}
