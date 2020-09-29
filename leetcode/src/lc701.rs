struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use crate::leetcode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
	pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
		if root.is_none() {
			return Some(Rc::new(RefCell::new(TreeNode::new(val))));
		}
		Solution::dfs(root.clone(), val);
		root
	}
	fn dfs(root: Option<Rc<RefCell<TreeNode>>>, val: i32) {
		let mut node = RefCell::borrow_mut(root.as_ref().unwrap());
		if val > node.val {
			if node.right.is_none() {
				node.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
			} else {
				Solution::dfs(node.right.clone(), val);
			}
		} else {
			if node.left.is_none() {
				node.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
			} else {
				Solution::dfs(node.left.clone(), val);
			}
		}
	}
}