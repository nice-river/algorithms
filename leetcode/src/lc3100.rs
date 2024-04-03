#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let num_bottles = 13;
        let num_exchange = 6;
        let ans = 15;
        assert_eq!(Solution::max_bottles_drunk(num_bottles, num_exchange), ans);
    }

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
    pub fn max_bottles_drunk(num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut ans = num_bottles;
        let mut num_empty = num_bottles;
        while num_empty >= num_exchange {
            ans += 1;
            num_empty -= num_exchange - 1;
            num_exchange += 1;
        }
        ans
    }
}
