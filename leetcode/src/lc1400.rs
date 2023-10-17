#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = String::from("qlkzenwmmnpkopu");
        let k = 15;
        assert!(Solution::can_construct(s, k));
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

use std::collections::HashMap;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut map = HashMap::new();
        for ch in s.as_bytes() {
            *map.entry(*ch).or_insert(0) += 1;
        }
        let mut twos = 0;
        let mut ones = 0;
        for (_, v) in map.into_iter() {
            twos += v / 2;
            ones += v % 2;
        }
        if ones > k {
            return false;
        } else if ones < k {
            return twos * 2 + ones >= k;
        } else {
            return true;
        }
    }
}
