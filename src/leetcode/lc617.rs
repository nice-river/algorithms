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
	pub fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
		if t1.is_none() && t2.is_none() {
			None
		} else if t1.is_some() && t2.is_some() {
			let t1 = RefCell::borrow(t1.as_ref().unwrap());
			let mut node = TreeNode::new(t1.val);
			node.left = Solution::merge_trees(t1.left.clone(), None);
			node.right = Solution::merge_trees(t1.right.clone(), None);
			Some(Rc::new(RefCell::new(node)))
		} else if t1.is_none() && t2.is_some() {
			let t2 = RefCell::borrow(t2.as_ref().unwrap());
			let mut node = TreeNode::new(t2.val);
			node.left = Solution::merge_trees(None, t2.left.clone());
			node.right = Solution::merge_trees(None, t2.right.clone());
			Some(Rc::new(RefCell::new(node)))
	 	} else {
			let t1 = RefCell::borrow(t1.as_ref().unwrap());
			let t2 = RefCell::borrow(t2.as_ref().unwrap());
			let mut node = TreeNode::new(t1.val + t2.val);
			node.left = Solution::merge_trees(t1.left.clone(), t2.left.clone());
			node.right = Solution::merge_trees(t1.right.clone(), t2.right.clone());
			Some(Rc::new(RefCell::new(node)))
		}
	}
}