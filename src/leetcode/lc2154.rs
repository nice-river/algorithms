#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, mut original: i32) -> i32 {
        let mut set = HashSet::new();
        for num in &nums {
            set.insert(*num);
        }
        while set.contains(&original) {
            original *= 2;
        }
        original
    }
}
