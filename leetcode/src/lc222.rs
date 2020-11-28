struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
		let mut root = TreeNode::new(1);
		root.left = left;
		let root = Some(Rc::new(RefCell::new(root)));
		assert_eq!(Solution::count_nodes(root), 2);
	}
}

use crate::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
	pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		if root.is_none() {
			return 0;
		}

		let mut level = 0;
		let mut most_left = root.clone();
		while let Some(node) = most_left {
			level += 1;
			most_left = RefCell::borrow(&node).left.clone();
		}
		if level == 1 {
			return 1;
		}
		let (mut left, mut right) = (1 << (level - 1), 1 << level);
		while left + 1 < right {
			let mut mid = left + (right - left) / 2;
			if Solution::exist(root.clone(), level, mid) {
				left = mid;
			} else {
				right = mid;
			}
		}
		left
	}

	fn exist(root: Option<Rc<RefCell<TreeNode>>>, level: i32, idx: i32) -> bool {
		let mut root = root;
		let mut level = level - 2;
		while level >= 0 {
			if ((1 << level) & idx) != 0 {
				// let node: &Rc<RefCell<TreeNode>> = &root.unwrap();
				let node: &Rc<RefCell<TreeNode>> = &root.unwrap();
				root = RefCell::borrow(node).right.clone();
			} else {
				let node: &Rc<RefCell<TreeNode>> = &root.unwrap();
				root = RefCell::borrow(node).left.clone();
			}
			level -= 1;
		}
		root.is_some()
	}
}