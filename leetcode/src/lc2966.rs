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

use core::num;

use crate::*;

struct Solution {}

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n % 3 != 0 {
            return vec![];
        }
        nums.sort();
        let mut ans = vec![];
        for i in (0..n).step_by(3) {
            if nums[i + 2] - nums[i] <= k {
                ans.push(vec![nums[i], nums[i + 1], nums[i + 2]]);
            } else {
                return vec![];
            }
        }
        ans
    }
}
