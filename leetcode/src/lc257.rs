use crate::leetcode::TreeNode;

struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;


impl Solution {
	pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
		if root.is_none() {
			vec![]
		} else {
			let mut ans = vec![];
			let mut path = vec![];
			Solution::dfs(root, &mut path, &mut ans);
			ans
		}
	}

	fn dfs(root: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<String>, ans: &mut Vec<String>) {
		if let Some(root) = &root {
			let node = RefCell::borrow(root);
			path.push(node.val.to_string());
			if node.left.is_none() && node.right.is_none() {
				ans.push(path.join("->"));
			}
			if let Some(left) = &node.left {
				Solution::dfs(Some(left.clone()), path, ans);
			}
			if let Some(right) = &node.right {
				Solution::dfs(Some(right.clone()), path, ans);
			}
			path.pop();
		}
	}
}
