#[cfg(test)]
mod tests {
    #[test]
    fn test0() {}
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let n = nums.len();
        let mut head = HashSet::new();
        let mut tail = HashSet::new();
        for i in 0..n {
            head.clear();
            tail.clear();
            for j in 0..=i {
                head.insert(nums[j]);
            }
            for j in i + 1..n {
                tail.insert(nums[j]);
            }
            ans.push((head.len() - tail.len()) as i32);
        }
        ans
    }
}
