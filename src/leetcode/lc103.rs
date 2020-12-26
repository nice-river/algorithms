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
	pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		if root.is_none() {
			return ans;
		}
		let mut que = vec![root];
		let mut cur = 0;
		while !que.is_empty() {
			let mut nxt = vec![];
			ans.push(vec![]);
			for node in que.iter() {
				if let Some(node) = node {
					let node = RefCell::borrow(node);
					ans.last_mut().unwrap().push(node.val);
					if node.left.is_some() {
						nxt.push(node.left.clone());
					}
					if node.right.is_some() {
						nxt.push(node.right.clone())
					}
				}
			}
			if cur == 1 {
				ans.last_mut().unwrap().reverse();
			}
			que = nxt;
			cur ^= 1;
		}
		ans
	}
}