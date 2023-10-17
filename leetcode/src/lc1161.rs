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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut maxi = i32::MIN;
        let mut ans = 0;

        let mut nodes = vec![vec![], vec![]];
        let mut idx = 0;
        nodes[0].push(root.unwrap());
        let mut layer = 1;
        while !nodes[idx].is_empty() {
            let mut val = 0;
            while let Some(node) = nodes[idx].pop() {
                let node = node.borrow();
                val += node.val;
                if let Some(ch) = node.left.as_ref() {
                    nodes[idx ^ 1].push(ch.clone());
                }
                if let Some(ch) = node.right.as_ref() {
                    nodes[idx ^ 1].push(ch.clone());
                }
            }
            if val > maxi {
                maxi = val;
                ans = layer;
            }
            layer += 1;
            idx ^= 1;
        }
        ans
    }
}
