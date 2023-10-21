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
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec;

impl Solution {
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        let mut pos = HashMap::new();
        for (i, &n) in voyage.iter().enumerate() {
            pos.insert(n, i);
        }
        let mut ans = vec![];
        if Self::dfs(&root, &voyage, 0, voyage.len(), &mut ans, &pos) {
            ans
        } else {
            vec![-1]
        }
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        voyage: &Vec<i32>,
        start: usize,
        end: usize,
        ans: &mut Vec<i32>,
        position: &HashMap<i32, usize>,
    ) -> bool {
        if let Some(root) = root.as_ref() {
            if start == end {
                return false;
            }
            let val = root.borrow().val;
            if val != voyage[start] {
                return false;
            }
            let left = &root.borrow().left;
            let right = &root.borrow().right;
            let mut left_pos = None;
            let mut right_pos = None;
            if let Some(left) = left {
                let left_val = left.borrow().val;
                if position.get(&left_val).is_none() {
                    return false;
                }
                left_pos = position.get(&left_val);
            }
            if let Some(right) = right {
                let right_val = right.borrow().val;
                if position.get(&right_val).is_none() {
                    return false;
                }
                right_pos = position.get(&right_val);
            }
            match (left_pos, right_pos) {
                (None, None) => {
                    if start + 1 != end {
                        return false;
                    }
                    return true;
                }
                (Some(left_pos), None) => {
                    if *left_pos != start + 1 || *left_pos <= start || *left_pos >= end {
                        return false;
                    }
                    return Self::dfs(left, voyage, *left_pos, end, ans, position);
                }
                (None, Some(right_pos)) => {
                    if *right_pos != start + 1 || *right_pos <= start || *right_pos >= end {
                        return false;
                    }
                    return Self::dfs(right, voyage, *right_pos, end, ans, position);
                }
                (Some(left_pos), Some(right_pos)) => {
                    if *left_pos <= start
                        || *left_pos >= end
                        || *right_pos <= start
                        || *right_pos >= end
                    {
                        return false;
                    }
                    if *left_pos == start + 1 {
                        if !Self::dfs(left, voyage, *left_pos, *right_pos, ans, position) {
                            return false;
                        }
                        return Self::dfs(right, voyage, *right_pos, end, ans, position);
                    } else if *right_pos == start + 1 {
                        ans.push(val);
                        if !Self::dfs(left, voyage, *left_pos, end, ans, position) {
                            return false;
                        }
                        return Self::dfs(right, voyage, *right_pos, *left_pos, ans, position);
                    } else {
                        return false;
                    }
                }
            }
        } else if start != end {
            return false;
        }
        true
    }
}
