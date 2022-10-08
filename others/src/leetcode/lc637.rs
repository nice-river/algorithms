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
use std::mem;

impl Solution {
	pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
		let mut cur = Vec::new();
		let mut nxt = Vec::new();
		let mut ans = Vec::new();
		if root.is_none() {
			return ans;
		}
		cur.push(root.unwrap().clone());
		while !cur.is_empty() {
			let mut v = 0f64;
			for node in &cur {
				let node = RefCell::borrow(&node);
				v += node.val as f64;
				if node.left.is_some() {
					nxt.push(node.left.as_ref().unwrap().clone());
				}
				if node.right.is_some() {
					nxt.push(node.right.as_ref().unwrap().clone());
				}
			}
			ans.push(v / cur.len() as f64);
			mem::swap(&mut cur, &mut nxt);
			nxt.clear();
		}
		ans
	}
}