#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn score_difference(nums: Vec<i32>) -> i32 {
        let mut x = vec![0, 0];
        let mut p = 0;
        for (i, num) in nums.into_iter().enumerate() {
            if (i + 1) % 6 == 0 {
                p ^= 1;
            }
            if num % 2 == 1 {
                p ^= 1;
            }
            x[p] += num;
        }
        
        x[0] - x[1]
    }
}