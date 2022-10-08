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
	pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		let mut ans = vec![];
		if root.is_none() {
			return ans;
		}
		let mut stk = vec![];
		stk.push(root.unwrap().clone());
		while !stk.is_empty() {
			let node = stk.pop();
			if let Some(node) = &node {
				let node= RefCell::borrow(node);
				ans.push(node.val);
				if let Some(right) = &node.right {
					stk.push(right.clone());
				}
				if let Some(left) = &node.left {
					stk.push(left.clone());
				}
			}
		}
		ans
	}
}
