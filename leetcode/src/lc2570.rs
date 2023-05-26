use core::num;

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
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            if nums1[i][0] == nums2[j][0] {
                ans.push(vec![nums1[i][0], nums1[i][1] + nums2[j][1]]);
                i += 1;
                j += 1;
            } else if nums1[i][0] < nums2[j][0] {
                ans.push(vec![nums1[i][0], nums1[i][1]]);
                i += 1;
            } else {
                ans.push(vec![nums2[j][0], nums2[j][1]]);
                j += 1;
            }
        }
        while i < nums1.len() {
            ans.push(vec![nums1[i][0], nums1[i][1]]);
            i += 1;
        }
        while j < nums2.len() {
            ans.push(vec![nums2[j][0], nums2[j][1]]);
            j += 1;
        }
        ans
    }
}
