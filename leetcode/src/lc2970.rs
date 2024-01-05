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
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        for i in 0..n {
            for j in i + 1..=n {
                let mut f = true;
                let mut last = i32::MIN;
                for t in 0..i {
                    if nums[t] <= last {
                        f = false;
                        break;
                    }
                    last = nums[t];
                }
                for t in j..n {
                    if nums[t] <= last {
                        f = false;
                        break;
                    }
                    last = nums[t];
                }
                if f {
                    ans += 1;
                }
            }
        }
        ans
    }
}