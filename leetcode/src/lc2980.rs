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
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        let mut cnt = 0;
        for num in nums {
            if num % 2 == 0 {
                cnt += 1;
            }
        }
        cnt >= 2
    }
}