#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "a".to_owned();
        let t = "aa".to_owned();
        let ans = "".to_owned();
        assert_eq!(Solution::min_window(s, t), ans);
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
    pub fn min_window(s: String, t: String) -> String {
        let mut hs = HashMap::new();
        let mut ht = HashMap::new();
        let mut set = HashSet::new();
        for c in t.as_bytes() {
            *ht.entry(*c).or_insert(0) += 1;
        }
        for k in ht.iter() {
            set.insert(*k.0);
        }

        let mut p = 0;
        let mut q = 0;
        let s = s.as_bytes();
        let mut ans = (0, usize::MAX);
        while q < s.len() {
            *hs.entry(s[q]).or_insert(0) += 1;
            if hs.get(&s[q]).unwrap_or(&0) >= ht.get(&s[q]).unwrap_or(&0) {
                set.remove(&s[q]);
            }
            if set.len() == 0 && q - p + 1 < ans.1 {
                ans = (p, q - p + 1);
            }
            while p < q {
                let g = ht.get(&s[p]);
                if g.is_none() {
                    *hs.entry(s[p]).or_default() -= 1;
                    p += 1;
                    if set.len() == 0 && q - p + 1 < ans.1 {
                        ans = (p, q - p + 1);
                    }
                } else {
                    let g = g.unwrap();
                    if hs.get(&s[p]).unwrap_or(&0) > g {
                        *hs.entry(s[p]).or_default() -= 1;
                        p += 1;
                        if set.len() == 0 && q - p + 1 < ans.1 {
                            ans = (p, q - p + 1);
                        }
                    } else {
                        break;
                    }
                }
            }
            q += 1;
        }
        if set.len() == 0 && q - p + 1 < ans.1 {
            ans = (p, q - p + 1);
        }
        if ans.1 == usize::MAX {
            "".to_owned()
        } else {
            std::str::from_utf8(&s[ans.0..ans.0 + ans.1]).unwrap().to_owned()
        }
    }
}