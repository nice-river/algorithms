#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 3, 5, 7];
        let ans = vec![-1, 1, 4, 3];
        assert_eq!(Solution::min_bitwise_array(nums), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        for num in nums {
            if num == 2 {
                ans.push(-1);
            } else {
                // num = xxx111
                let t = (num + 1) >> 1; // xxx100
                let a = t ^ (t - 1); // 000111
                let b = t & a; // 000100
                ans.push(num & (!b));
            }
        }
        ans
    }
}
