#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}

struct Solution {}


impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        let mut k = nums1.len();
        while m > 0 && n > 0 {
            if nums1[m - 1] > nums2[n - 1] {
                nums1[k - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[k - 1] = nums2[n - 1];
                n -= 1;
            }
            k -= 1;
        }
        while m > 0 {
            nums1[k - 1] = nums1[m - 1];
            m -= 1;
            k -= 1;
        }
        while n > 0 {
            nums1[k - 1] = nums2[n - 1];
            n -= 1;
            k -= 1;
        }
    }
}