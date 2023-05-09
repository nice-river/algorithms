#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for &c in &candy_type {
            set.insert(c);
        }
        set.len().min(candy_type.len() / 2) as i32
    }
}
