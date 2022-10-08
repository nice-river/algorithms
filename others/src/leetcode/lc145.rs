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
	pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		let mut ans = vec![];
		let mut stk = vec![];
		if root.is_none() {
			return ans;
		}
		stk.push((root.unwrap(), 0));

		while !stk.is_empty() {
			let (node, cnt) = stk.pop().unwrap();
			if cnt == 0 {
				stk.push((node.clone(), 1));
				if let Some(right) = &RefCell::borrow(&node).right {
					stk.push((right.clone(), 0));
				}
				if let Some(left) = &RefCell::borrow(&node).left {
					stk.push((left.clone(), 0));
				}
			} else if cnt == 1 {
				ans.push(RefCell::borrow(&node).val);
			}
		}
		ans
	}
}