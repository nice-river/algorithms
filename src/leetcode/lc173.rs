#[cfg(test)]
mod tests {
	use super::*;
	use std::cell::Ref;

	#[test]
	fn test() {
		let mut x = TreeNode::new(15);
		x.left_node(TreeNode::new(9));
		x.right_node(TreeNode::new(20));
		let mut root = TreeNode::new(7);
		root.left_node(TreeNode::new(3));
		root.right_node(x);
		let mut iter = BSTIterator::new(Some(Rc::new(RefCell::new(root))));
		while iter.has_next() {
			println!("{}", iter.next());
		}
	}
}

struct Solution {}

use crate::leetcode::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

struct BSTIterator {
	stk: Vec<(Rc<RefCell<TreeNode>>, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
	fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
		if root.is_none() {
			Self {
				stk: vec![],
			}
		} else {
			Self {
				stk: vec![(root.unwrap(), 0)],
			}
		}
	}

	fn next(&mut self) -> i32 {
		assert!(self.has_next());
		let (mut node, _) = self.stk.pop().unwrap();
		let x = RefCell::borrow(&node).val;
		let right = RefCell::borrow_mut(&mut node).right.take();
		if let Some(right) = right {
			self.stk.push((right, 0));
		}
		x
	}

	fn has_next(&mut self) -> bool {
		while let Some((node, x)) = self.stk.last_mut() {
			if *x != 0 {
				break;
			}
			*x += 1;
			let left = RefCell::borrow_mut(node).left.take();
			if left.is_some() {
				self.stk.push((left.unwrap(), 0));
			}
		}
		!self.stk.is_empty()
	}
}
