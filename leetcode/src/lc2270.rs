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
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + nums[i] as i64;
        }
        let mut ans = 0;
        for i in 1..n {
            if s[i] >= s[n] - s[i] {
                ans += 1;
            }
        }
        ans
    }
}
