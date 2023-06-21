#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let node = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        Solution::insert_into_max_tree(node, 2);
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
    pub fn insert_into_max_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            if root.borrow().val < val {
                let node = RefCell::new(TreeNode::new(val));
                node.borrow_mut().left = Some(root);
                Some(Rc::new(node))
            } else {
                let right = root.borrow_mut().right.take();
                root.borrow_mut().right = Self::insert_into_max_tree(right, val);
                Some(root)
            }
        } else {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}
