#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}
use crate::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::borrow::Borrow;

impl Solution {
	pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
		if root.is_none() {
			return None
		}
		let val = RefCell::borrow(root.as_ref().unwrap()).val;
		let mut dst = Rc::new(RefCell::new(TreeNode::new(val)));
		Solution::dfs(root.unwrap(), &mut dst);
		Some(dst)
	}

	fn dfs(src: Rc<RefCell<TreeNode>>, dst: &mut Rc<RefCell<TreeNode>>) {
		let src_left = &RefCell::borrow(&src).left;
		if src_left.is_some() {
			let val = RefCell::borrow(&src_left.as_ref().unwrap()).val;
			RefCell::borrow_mut(dst).right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
			Solution::dfs(src_left.as_ref().unwrap().clone(), RefCell::borrow_mut(dst).right.as_mut().unwrap());
		}
		let src_right = &RefCell::borrow(&src).right;
		if src_right.is_some() {
			let val = RefCell::borrow(&src_right.as_ref().unwrap()).val;
			RefCell::borrow_mut(dst).left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
			Solution::dfs(src_right.as_ref().unwrap().clone(), RefCell::borrow_mut(dst).left.as_mut().unwrap());
		}
	}
}
