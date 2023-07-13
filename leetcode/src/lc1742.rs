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
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut map = HashMap::new();
        for mut i in low_limit..=high_limit {
            let mut t = 0;
            while i != 0 {
                t += i % 10;
                i /= 10;
            }
            *map.entry(t).or_insert(0) += 1;
        }
        let mut ans = 0;
        for (_, v) in map.into_iter() {
            ans = ans.max(v);
        }
        ans
    }
}
