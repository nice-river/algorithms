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
    pub fn number_of_weeks(mut milestones: Vec<i32>) -> i64 {
        milestones.sort_by_key(|e| std::cmp::Reverse(*e));
        let mut ans = 0;
        for e @ (&v as i64) in milestones.iter().skip(1) {
            ans += v as i64;
        }
        if ans >= milestones[0] as i64 - 1 {
            ans + milestones[0] as i64
        } else {
            ans * 2 + 1
        }
    }
}
