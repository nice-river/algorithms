#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use crate::leetcode::leetcode::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root.as_ref() {
            let node = root.borrow();
            if node.val == val {
                Some(root.clone())
            } else if node.val < val {
                Solution::search_bst(node.right.clone(), val)
            } else {
                Solution::search_bst(node.left.clone(), val)
            }
        } else {
            None
        }
    }
}
