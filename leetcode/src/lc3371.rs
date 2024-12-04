#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 3, 5, 10];
        let ans = 10;
        assert_eq!(Solution::get_largest_outlier(nums), ans);
    }
}

use crate::*;

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let s: i32 = nums.iter().sum();
        let mut map = HashMap::new();
        for &num in &nums {
            *map.entry(num).or_insert(0) += 1;
        }
        for &num in &nums {
            let t = s - num;
            if t % 2 == 0 {
                if let Some(c) = map.get(&(t / 2)) {
                    if num == t / 2 {
                        if *c >= 2 {
                            ans = ans.max(num);
                        }
                    } else if *c >= 1 {
                        ans = ans.max(num);
                    }
                }
            }
        }
        ans
    }
}
