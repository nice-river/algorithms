struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {

    }
}

use std::collections::BTreeMap;
use std::ops::Bound::Included;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if nums.len() == 0 || k == 0 {
            return false;
        }
        let k = k as usize;
        let t = t as i64;
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut ordered = BTreeMap::new();
        for i in 0..k.min(nums.len()) {
            *ordered.entry(nums[i]).or_insert(0) += 1;
        }
        let mut pre = *ordered.iter().next().unwrap().0 - t - 1;
        for (&k, &v) in ordered.iter() {
            if v >= 2 {
                return true;
            }
            if (k - pre).abs() <= t {
                return true;
            }
            pre = k;
        }
        for i in k..nums.len() {
            let res = ordered.range((Included(&nums[i] - t), Included(&nums[i] + t)));
            for _ in res {
                return true;
            }
            *ordered.entry(nums[i - k]).or_insert(1) -= 1;
            if ordered.get(&nums[i - k]).unwrap() == &0 {
                ordered.remove(&nums[i - k]);
            }
            *ordered.entry(nums[i]).or_insert(0) += 1;
        }
        false
    }
}