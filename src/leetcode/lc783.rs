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
	pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut prev = None;
        let mut mini = i32::MAX;
        Solution::dfs(root, &mut prev, &mut mini);
		mini
	}

	fn dfs(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, mini: &mut i32) {
        if let Some(node) = root {
			let node = RefCell::borrow(&node);
			if let Some(left) = node.left.clone() {
                Solution::dfs(node.left.clone(), prev, mini);
			}
			if let Some(p) = prev {
				*mini = (*mini).min((node.val - *p).abs());
			}
			*prev = Some(node.val);
			if let Some(right) = node.right.clone() {
				Solution::dfs(node.right.clone(), prev, mini);
			}
		}
	}
}