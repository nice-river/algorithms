#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![50,28,48];
        let ans = 5;
        assert_eq!(Solution::sum_digit_differences(nums), ans);
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
    pub fn sum_digit_differences(mut nums: Vec<i32>) -> i64 {
        let mut digits;
        let mut ans = 0;
        for _ in 0..11 {
            digits = vec![0; 10];
            for num in nums.iter_mut() {
                digits[(*num % 10) as usize] += 1; 
                *num /= 10;
            }
            let mut all: i64 = digits.iter().sum();
            for v in digits {
                ans += v * (all - v);
                all -= v;
            }
        }
        ans
    }
}
