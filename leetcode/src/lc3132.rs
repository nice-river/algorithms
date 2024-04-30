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
    pub fn minimum_added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort();
        nums2.sort();
        let mut ans = i32::MAX;
        for i in 0..nums1.len() {
            for j in i + 1..nums1.len() {
                let mut p = 0;
                let mut q = 0;
                let mut diff = None;
                while p < nums1.len() && q < nums2.len() {
                    while p == i || p == j {
                        p += 1;
                    }
                    match diff {
                        Some(d) => {
                            if d != nums2[q] - nums1[p] {
                                diff = None;
                                break;
                            }
                        }
                        None => {
                            diff = Some(nums2[q] - nums1[p]);
                        }
                    }
                    p += 1;
                    q += 1;
                }
                if let Some(x) = diff {
                    ans = ans.min(x);
                }
            }
        }
        ans
    }
}
