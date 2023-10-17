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

use crate::*;

struct Solution {}

use std::collections::BTreeSet;

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut set = BTreeSet::new();
        for (i, &num) in nums.iter().enumerate() {
            set.insert((num, i));
        }
        while let Some(&(num, i)) = set.iter().next() {
            ans += num as i64;
            if i > 0 {
                set.remove(&(nums[i - 1], i - 1));
            }
            if i + 1 < nums.len() {
                set.remove(&(nums[i + 1], i + 1));
            }
            set.remove(&(num, i));
        }
        ans
    }
}
