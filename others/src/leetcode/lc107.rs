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
	pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
		let mut ans = Vec::new();
		if root.is_none() {
			return ans;
		}
		let mut que = VecDeque::new();
		que.push_back(root.unwrap().clone());
		while !que.is_empty() {
			let cur_len = que.len();
			ans.push(Vec::new());
			for _ in 0..cur_len {
				let node = que.pop_front().unwrap();
				let node = RefCell::borrow(&node);
				ans.last_mut().unwrap().push(node.val);
				if let Some(left) = &node.left {
					que.push_back(left.clone());
				}
				if let Some(right) = &node.right {
					que.push_back(right.clone());
				}
			}
		}
		ans.reverse();
		ans
	}
}