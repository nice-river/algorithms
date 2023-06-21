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
    pub fn min_deletions(s: String) -> i32 {
        let mut map = HashMap::new();
        for ch in s.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }
        let mut maxi = 0;
        let mut cnts = HashMap::new();
        for (_, cnt) in map.into_iter() {
            *cnts.entry(cnt).or_insert(0) += 1;
            maxi = maxi.max(cnt);
        }
        let mut ans = 0;
        for i in (1..=maxi).rev() {
            let &t = cnts.get(&i).unwrap_or(&0);
            if t > 1 {
                ans += t - 1;
                *cnts.entry(i - 1).or_insert(0) += t - 1;
            }
        }
        ans
    }
}
