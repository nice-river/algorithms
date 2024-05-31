#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums1 = vec![1, 3, 4];
        let nums2 = vec![1, 3, 4];
        let k = 1;
        let ans = 5;
        assert_eq!(Solution::number_of_pairs(nums1, nums2, k), ans);
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

use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut cnts1 = HashMap::new();
        let mut cnts2 = HashMap::new();
        let mut maxi = 0;
        for num in nums1 {
            if num % k == 0 {
                *cnts1.entry(num / k).or_insert(0) += 1;
                maxi = maxi.max(num / k);
            }
        }
        for num in nums2 {
            *cnts2.entry(num).or_insert(0) += 1;
        }
        let mut ans = 0;
        for (num, a) in cnts2.into_iter() {
            let mut y = num;
            while y <= maxi {
                if let Some(&b) = cnts1.get(&y) {
                    ans += a * b;
                }
                y += num;
            }
        }
        ans
    }
}
