#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn min_number(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort();
        nums2.sort();
        for &a in nums1.iter() {
            for &b in nums2.iter() {
                if a == b {
                    return a;
                }
            }
        }
        if nums1[0] < nums2[0] {
            nums1[0] * 10 + nums2[0]
        } else {
            nums2[0] * 10 + nums1[0]
        }
    }
}
