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
use std::cmp::min;

impl Solution {
	pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut pre = None;
		let mut ans = std::i32::MAX;
		Solution::dfs(&root, &mut pre, &mut ans);
		ans
	}

	fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, pre: &mut Option<i32>, ans: &mut i32) {
		if let Some(root) = root {
			let root = RefCell::borrow(root);
			Solution::dfs(&root.left, pre, ans);
			if let Some(val) = pre {
				*ans = min(ans.clone(), root.val - val.clone());
			}
			*pre = Some(root.val);
			Solution::dfs(&root.right, pre, ans);
		}
	}
}