#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![4, 1, 1, 2];
        let ans = vec![8];
        assert_eq!(Solution::merge_adjacent(nums), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![8, 3, 3, 2, 2, 1, 1, 6];
        let ans = vec![8, 6, 4, 2, 6];
        assert_eq!(Solution::merge_adjacent(nums), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 1, 1, 2];
        let ans = vec![4, 2];
        assert_eq!(Solution::merge_adjacent(nums), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![2, 2, 1, 1];
        let ans = vec![4, 2];
        assert_eq!(Solution::merge_adjacent(nums), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn merge_adjacent(nums: Vec<i32>) -> Vec<i64> {
        if nums.is_empty() {
            return vec![];
        }
        let mut stack = vec![];
        for num in nums {
            let num = num as i64;
            if stack.is_empty() {
                stack.push(num);
            } else if stack.last().unwrap() != &num {
                stack.push(num);
            } else {
                let mut k = num;
                while let Some(x) = stack.pop() {
                    if x != k {
                        stack.push(x);
                        break;
                    } else {
                        k = k * 2;
                    }
                }
                stack.push(k);
            };
        }
        stack
    }
}
