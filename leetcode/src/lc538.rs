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
use std::cell::RefCell;

impl Solution {
	pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
		Solution::dfs(&mut root.clone(), 0);
		root
	}

	fn dfs(root: &mut Option<Rc<RefCell<TreeNode>>>, su: i32) -> i32 {
		if root.is_none() {
			return 0;
		}
		let mut node = RefCell::borrow_mut(&mut root.as_ref().unwrap());
		let mut s = node.val;
		let r = Solution::dfs(&mut node.right, su);
		s += r;
		node.val += su + r;
		let k = node.val;
		s += Solution::dfs(&mut node.left, k);
		s
	}
}