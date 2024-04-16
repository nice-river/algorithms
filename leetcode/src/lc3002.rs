#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![2, 2, 2, 2];
        let ans = 3;
        assert_eq!(Solution::maximum_set_size(nums1, nums2), ans);
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

use std::collections::HashSet;

impl Solution {
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        let n = nums1.len();
        for num in nums1 { set1.insert(num); }
        for num in nums2 { set2.insert(num); }
        let mut c1 = 0;
        let mut c2 = 0;
        let mut s =  HashSet::new();
        for a in &set1 {
            if !set2.contains(a) {
                c1 += 1;
                s.insert(*a);
            }
            if c1 >= n / 2 {
                break;
            }
        }
        for b in &set2 {
            if !set1.contains(b) {
                c2 += 1;
                s.insert(*b);
            }
            if c2 >= n / 2 {
                break;
            }
        }
        for a in &set1 {
            if c1 >= n / 2 {
                break;
            }
            if !s.contains(a) {
                c1 += 1;
                s.insert(*a);
            }
        }
        for b in &set2 {
            if c2 >= n / 2 {
                break;
            }
            if !s.contains(b) {
                c2 += 1;
                s.insert(*b);
            }
        }
        s.len() as i32
    }
}