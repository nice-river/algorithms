struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use crate::leetcode::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
		Solution::dfs(root, None)
    }

	fn dfs(mut root: Option<Rc<RefCell<TreeNode>>>, tail: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
		if let Some(root) = root {
            let left = root.borrow_mut().left.take();
            let t = Solution::dfs(root.borrow_mut().right.take(), tail);
			root.borrow_mut().right = t;
			Solution::dfs(left, Some(root))
		} else {
			tail
		}
	}

}