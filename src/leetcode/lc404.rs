struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sol = Solution {};
	}
}

use crate::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
	pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		let mut ans = 0;
		if root.is_none() {
			return ans;
		}
		let mut que = VecDeque:: new();
		que.push_back((root.unwrap(), false));
		while !que.is_empty() {
			let (node, is_left) = que.pop_front().unwrap();
			let node = RefCell::borrow(&node);
			if node.left.is_some() {
				que.push_back((node.left.as_ref().unwrap().clone(), true));
			}
			if node.right.is_some() {
				que.push_back((node.right.as_ref().unwrap().clone(), false));
			} else if is_left && node.left.is_none() {
				ans += node.val;
			}
		}
		ans
	}
}
