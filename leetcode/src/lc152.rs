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
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let mut neg = 0;
        let mut s = 1;
        for num in nums {
            if num == 0 {
                ans = ans.max(0);
                neg = 0;
                s = 1;
            } else {
                s *= num;
                ans = ans.max(s);
                if s < 0 {
                    if neg == 0 {
                        neg = s;
                    } else {
                        ans = ans.max(s / neg);
                    }
                }
            }
        }
        ans
    }
}
