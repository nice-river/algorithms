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
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut cnt = 0;
        for num in nums {
            if cnt == 0 {
                ans = num;
                cnt = 1;
            } else if num == ans {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }
        ans
    }
}
