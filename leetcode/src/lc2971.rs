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
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        let mut ans = -1;
        nums.sort();
        let mut c = (nums[0] + nums[1]) as i64;
        for i in 2..nums.len() {
            if (nums[i] as i64) < c {
                ans = c + nums[i] as i64;
            }
            c += nums[i] as i64;
        }
        ans
    }
}