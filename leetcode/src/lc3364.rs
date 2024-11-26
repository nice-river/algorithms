#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut ans: Option<i32> = None;
        for k in l as usize..=r as usize {
            for i in 0..nums.len() + 1 - k {
                let mut s = 0;
                for j in 0..k {
                    s += nums[i + j];
                }
                if s > 0 {
                    ans = Some(ans.map_or(s, |v| v.min(s)));
                }
            }
        }
        ans.unwrap_or(-1)
    }
}
