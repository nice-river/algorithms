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

use std::collections::BTreeSet;

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut set = BTreeSet::new();
        for i in 0..nums1.len() {
            set.insert((nums1[i], i));
        }
        let mut ans = vec![0; nums1.len()];
        for i in 0..nums2.len() {
            let mut r = set.range((nums2[i] + 1, 0)..);
            let k = if let Some(p) = r.next() {
                ans[i] = p.0;
                *p
            } else {
                let p = set.iter().next().unwrap();
                ans[i] = p.0;
                *p
            };
            set.remove(&k);
        }
        ans
    }
}
