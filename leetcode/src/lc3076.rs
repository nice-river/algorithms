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

use std::collections::HashSet;

impl Solution {
    pub fn shortest_substrings(arr: Vec<String>) -> Vec<String> {
        let mut ans = vec![];
        let arr = arr.into_iter().map(|s| s.into_bytes()).collect::<Vec<_>>();
        let mut hashes = vec![HashSet::new(); arr.len()];
        for (i, s) in arr.iter().enumerate() {
            for x in 0..s.len() {
                for y in x + 1..=s.len() {
                    let v = &s[x..y];
                    hashes[i].insert(v);
                }
            }
        }
        for (i, s) in arr.iter().enumerate() {
            let mut found: Option<&[u8]> = None;
            for l in 1..=s.len() {
                for x in 0..=s.len() - l {
                    let v = &s[x..x + l];
                    let mut occur = false;
                    for (j, hash) in hashes.iter().enumerate() {
                        if i != j {
                            if hash.contains(v) {
                                occur = true;
                                break;
                            }
                        }
                    }
                    if !occur {
                        if let Some(f) = found {
                            found = Some(f.min(v));
                        } else {
                            found = Some(v);
                        }
                    }
                }
                if found.is_some() {
                    break;
                }
            }
            if let Some(f) = found {
                ans.push(std::str::from_utf8(f).unwrap().to_string());
            } else {
                ans.push(String::new());
            }
        }
        ans
    }
}
