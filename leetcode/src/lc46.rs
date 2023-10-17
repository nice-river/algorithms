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
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        Self::dfs(&mut nums, 0, &mut ans);
        ans
    }

    fn dfs(nums: &mut Vec<i32>, idx: usize, ans: &mut Vec<Vec<i32>>) {
        if idx == nums.len() {
            ans.push(nums.clone());
        }
        for i in idx..nums.len() {
            nums.swap(i, idx);
            Self::dfs(nums, idx + 1, ans);
            nums.swap(i, idx);
        }
    }
}
