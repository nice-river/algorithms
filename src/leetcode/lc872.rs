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
	pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return if root1.is_some() && root2.is_some() {
			let mut leafs1 = vec![];
			let mut leafs2 = vec![];
            Solution::dfs(root1.unwrap(), &mut leafs1);
			Solution::dfs(root2.unwrap(), &mut leafs2);
			leafs1 == leafs2
		} else {
			root1.is_none() && root2.is_none()
		}
	}

	fn dfs(root: Rc<RefCell<TreeNode>>, leafs: &mut Vec<i32>) {
        let mut root = root.as_ref().borrow_mut();
		if root.left.is_none() && root.right.is_none() {
            leafs.push(root.val);
			return ;
		}
        if let Some(left) = root.left.take() {
            Solution::dfs(left, leafs)
		}
		if let Some(right) = root.right.take() {
			Solution::dfs(right, leafs)
		}
	}
}
