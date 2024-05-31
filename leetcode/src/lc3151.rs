#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1];
        assert!(Solution::is_array_special(nums));
    }

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

use crate::*;

struct Solution {}

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut s = vec![0; nums.len() + 1];
        for i in 1..nums.len() {
            let t = (nums[i] % 2) ^ (nums[i - 1] % 2);
            s[i] += s[i - 1] + t as usize;
        }
        s[nums.len() - 1] - s[0] == nums.len() - 1
    }
}
