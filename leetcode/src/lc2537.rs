#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 1, 3, 1, 2, 2, 3, 3, 2, 2, 1, 1, 1, 3, 1];
        let k = 11;
        let ans = 21;
        assert_eq!(Solution::count_good(nums, k), ans);
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

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as i64;
        let n = nums.len();
        let mut left = 0;
        let mut right = 0;
        let mut p = 0i64;
        let mut ans = 0;
        let mut map = HashMap::new();
        while right < n {
            if p < k {
                let e = map.entry(nums[right]).or_insert(0);
                p += *e;
                *e += 1;
                right += 1;
            } else {
                ans += (n - right + 1) as i64;
                let &g = map.get(&nums[left]).unwrap();
                p -= g - 1;
                if g == 1 {
                    map.remove(&nums[left]);
                } else {
                    map.insert(nums[left], g - 1);
                }
                left += 1;
            }
        }
        while p >= k {
            ans += 1;
            let &g = map.get(&nums[left]).unwrap();
            p -= g - 1;
            if g == 1 {
                map.remove(&nums[left]);
            }
            left += 1;
        }
        ans
    }
}
