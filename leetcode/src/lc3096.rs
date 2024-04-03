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
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let mut left_score = 0;
        for i in 0..possible.len() {
            left_score += possible[i] * 2 - 1;
        }
        let mut cur_score = 0;
        for i in 0..possible.len() - 1 {
            cur_score += possible[i] * 2 - 1;
            left_score -= possible[i] * 2 - 1;
            if cur_score > left_score {
                return i as i32 + 1;
            }
        }
        -1
    }
}
