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

struct Solution {}

impl Solution {
    pub fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut ans = 1;
        let mut a = 1;
        let mut b = 1;
        for i in 1..n {
            let mut c = 1;
            let mut d = 1;
            if nums1[i] >= nums1[i - 1] {
                c = c.max(a + 1);
            }
            if nums1[i] >= nums2[i - 1] {
                c = c.max(b + 1);
            }
            if nums2[i] >= nums1[i - 1] {
                d = d.max(a + 1);
            }
            if nums2[i] >= nums2[i - 1] {
                d = d.max(b + 1);
            }
            ans = ans.max(c).max(d);
            a = c;
            b = d;
        }
        ans
    }
}
