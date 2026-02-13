#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn minimum_prefix_length(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }
        let mut i = nums.len() as i32 - 2;
        while i >= 0 {
            if nums[i as usize] >= nums[i as usize + 1] {
                break;
            }
            i -= 1;
        }
        i as i32 + 1
    }
}
