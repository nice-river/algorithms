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
    pub fn maximize_greatness(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ans = 0;
        let mut q = 0;
        for p in 0..nums.len() {
            while q < nums.len() && nums[q] == nums[p] {
                q += 1;
            }
            if q < nums.len() {
                ans += 1;
                q += 1;
            } else {
                break;
            }
        }
        ans
    }
}
