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

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut map = HashMap::new();
        for hour in hours {
            let hour = hour % 24;
            if let Some(v) = map.get(&((24 - hour) % 24)) {
                ans += *v;
            }
            *map.entry(hour).or_insert(0) += 1;
        }
        ans
    }
}
