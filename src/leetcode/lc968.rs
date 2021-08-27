use crate::leetcode::TreeNode;

struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

impl Solution {
	pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
		if let Some(root) = root.as_ref() {
			let mut vis = HashSet::new();
			Solution::dfs(root.clone(), 1, &mut vis);
			vis.len() as i32
		} else {
			0
		}
	}

	fn dfs(root: Rc<RefCell<TreeNode>>, idx: usize, vis: &mut HashSet<usize>) {
		let root = RefCell::borrow(&root);
		if root.left.is_some() {
			Solution::dfs(root.left.as_ref().unwrap().clone(), idx << 1, vis);
		}
		if root.right.is_some() {
			Solution::dfs(root.right.as_ref().unwrap().clone(), idx << 1 | 1, vis);
		}
		if vis.contains(&(idx << 1)) || vis.contains(&(idx << 1 | 1)) || vis.contains(&(idx >> 1)) || vis.contains(&idx) {
			return
		}
		vis.insert(idx >> 1);
	}
}