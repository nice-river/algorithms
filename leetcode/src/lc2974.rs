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

impl Solution {
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        nums.sort();
        for i in (0..nums.len()).step_by(2) {
            ans.push(nums[i + 1]);
            ans.push(nums[i]);
        }
        ans
    }
}
