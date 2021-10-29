#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let n = nums2.len();
        let mut next_big = vec![n; nums2.len()];
        let mut map = HashMap::new();
        for i in (0..n).rev() {
            map.insert(nums2[i], i);
            let mut j = i + 1;
            while j < n && nums2[j] < nums2[i] {
                j = next_big[j];
            }
            next_big[i] = j;
        }
        let mut ans = vec![-1; nums1.len()];
        for (num, e) in nums1.into_iter().zip(ans.iter_mut()) {
            *e = map
                .get(&num)
                .map(|&idx| {
                    if next_big[idx] == n {
                        -1
                    } else {
                        nums2[next_big[idx]]
                    }
                })
                .unwrap_or(-1);
        }
        ans
    }
}
