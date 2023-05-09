#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut starts = HashSet::new();
        paths.iter().for_each(|e| {
            starts.insert(e[0].clone());
        });
        for path in paths.iter() {
            if !starts.contains(&path[1]) {
                return path[1].clone();
            }
        }
        unreachable!();
    }
}
