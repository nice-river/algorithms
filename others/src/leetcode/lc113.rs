#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
		assert_eq!(Solution::path_sum(root, 1), vec![[1]]);
	}
}

struct Solution {}

use crate::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
	pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		if root.is_none() {
			return ans;
		}
		let mut path = vec![];
		Solution::dfs(root.unwrap(), 0, sum, &mut path, &mut ans);
		ans
	}

	fn dfs(root: Rc<RefCell<TreeNode>>, sum: i32, target: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
		let root = RefCell::borrow(&root);
		path.push(root.val);
		if root.left.is_none() && root.right.is_none() {
			if sum + root.val == target {
				ans.push(path.clone());
			}
		}
		if root.left.is_some() {
			Solution::dfs(root.left.as_ref().unwrap().clone(), sum + root.val, target, path, ans)
		}
		if root.right.is_some() {
			Solution::dfs(root.right.as_ref().unwrap().clone(), sum + root.val, target, path, ans)
		}
		path.pop();
	}
}