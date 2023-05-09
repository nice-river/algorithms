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
	pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut ans = 0;
		Solution::dfs(root.clone(), 0, &mut ans);
		ans
	}

	fn dfs(node: Option<Rc<RefCell<TreeNode>>>, cur: i32, ans: &mut i32) {
		if let Some(node) = &node {
			let node = RefCell::borrow(node);
			let cur = cur * 10 + node.val;
			let left = node.left.clone();
			let right = node.right.clone();
			if left.is_none() && right.is_none() {
				*ans += cur;
				return ;
			}
			Solution::dfs(node.left.clone(), cur, ans);
			Solution::dfs(node.right.clone(), cur, ans);
		}
	}
}