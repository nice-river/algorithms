#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn is_possible_to_rearrange(s: String, t: String, k: i32) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut map = HashMap::new();
        let k = k as usize;
        let n = s.len() / k;
        for i in 0..k {
            *map.entry(&s[n * i..n * (i + 1)]).or_insert(0) += 1;
        }
        for i in 0..k {
            if let Some(c) = map.get_mut(&t[n * i..n * (i + 1)]) {
                if *c > 0 {
                    *c -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}
