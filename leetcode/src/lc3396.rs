#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut set = HashSet::new();
        for (i, num) in nums.into_iter().rev().enumerate() {
            if set.contains(&num) {
                return ((n - (i + 1) + 3) / 3) as i32;
            }
            set.insert(num);
        }
        0
    }
}
