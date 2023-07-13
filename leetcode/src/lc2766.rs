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

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        for num in nums {
            set.insert(num);
        }
        for (from, to) in move_from.into_iter().zip(move_to.into_iter()) {
            set.remove(&from);
            set.insert(to);
        }
        let mut ans = set.into_iter().collect::<Vec<_>>();
        ans.sort();
        ans
    }
}
