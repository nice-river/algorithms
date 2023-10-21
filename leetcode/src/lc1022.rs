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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(root.as_ref(), 0, &mut ans);
        ans
    }

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, val: i32, ans: &mut i32) {
        if let Some(root) = root.as_ref() {
            let root = root.borrow();
            let val = (val << 1) + root.val;
            let left = root.left.as_ref();
            let right = root.right.as_ref();
            if left.is_none() && right.is_none() {
                *ans += val;
                return;
            }
            if left.is_some() {
                Self::dfs(left, val, ans);
            }
            if right.is_some() {
                Self::dfs(right, val, ans);
            }
        }
    }
}
