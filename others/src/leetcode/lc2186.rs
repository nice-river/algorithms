#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut ans = 0;
        let mut s_map = HashMap::new();
        let mut t_map = HashMap::new();
        for &c in s.as_bytes() {
            *s_map.entry(c).or_insert(0i32) += 1;
        }
        for &c in t.as_bytes() {
            *t_map.entry(c).or_insert(0) += 1;
        }
        for (k, &v) in s_map.iter() {
            ans += (v - *t_map.get(k).unwrap_or(&0)).abs();
        }
        for (k, &v) in t_map.iter() {
            if !s_map.contains_key(k) {
                ans += v
            }
        }
        ans
    }
}
