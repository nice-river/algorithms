struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;
	use std::cell::RefMut;

	#[test]
	fn test() {
		let left = TreeNode::new(2);
		let right = TreeNode::new(3);
		let mut root = TreeNode::new(2);
		root.left = Some(Rc::new(RefCell::new(left)));
		root.right = Some(Rc::new(RefCell::new(right)));
		let root = Some(Rc::new(RefCell::new(root)));
		let ans = 3;
		assert_eq!(Solution::find_second_minimum_value(root), ans);
	}
}

use crate::leetcode::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;

impl Solution {
	pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
			return -1;
		}
		let val = root.as_ref().unwrap().as_ref().borrow().val;
        Solution::dfs(root, val)
	}

	fn dfs(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
        if let Some(root) = root.as_ref() {
            let node = root.as_ref().borrow();
            if node.val > val {
				return node.val;
			}
			let l = Solution::dfs(node.left.clone(), val);
			let r = Solution::dfs(node.right.clone(), val);
			if l > val && r > val {
				return l.min(r);
			}
			l.max(r)
		} else {
            -1
		}
	}
}