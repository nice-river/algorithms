#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn dominant_indices(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut accu = 0;
        for (i, num) in nums.into_iter().rev().enumerate() {
            if num * i as i32 > accu {
                ans += 1;
            }
            accu += num;
        }
        ans as i32
    }
}
